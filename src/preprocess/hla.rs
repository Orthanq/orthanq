use anyhow::Result;
use derive_builder::Builder;

use csv::ReaderBuilder;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use tempfile::tempdir;
use tempfile::NamedTempFile;

#[derive(Builder, Clone)]
pub struct Caller {
    genome: PathBuf,
    reads: Vec<PathBuf>,
    haplotype_variants: PathBuf,
    bwa_index: Option<PathBuf>,
    vg_index: PathBuf,
    output: PathBuf,
    threads: String,
}

impl Caller {
    pub fn call(&self) -> Result<()> {
        let outdir = &self.output;

        let cargo_dir = env!("CARGO_MANIFEST_DIR");

        //create the folder first if it doesn't exist
        fs::create_dir_all(&outdir)?;

        //todo: consider caching for indexing.

        //create a temporary file for bwa index and execute bwa index

        // Create a directory inside of `std::env::temp_dir()`
        let temp_dir = tempdir()?;

        //linear genome index location by default is temporary
        let mut linear_genome_index = outdir.join("hs_genome");

        // if bwa index is provided, linear genome index has to change
        if let Some(bwa_genome_index) = &self.bwa_index {
            linear_genome_index = bwa_genome_index.clone();
            println!(
                "using input bwa index at: {}",
                linear_genome_index.display()
            );
        } else {
            println!("building bwa index at: {}", linear_genome_index.display());
            let index = {
                Command::new("bwa")
                    .arg("index")
                    .arg("-p")
                    .arg(&linear_genome_index)
                    .arg("-a")
                    .arg("bwtsw") //-a bwtsw' does not work for short genomes, lineage quantification?
                    .arg(self.genome.clone())
                    .status()
                    .expect("failed to execute indexing process")
            };
            println!("The index was created successfully: {}", index);
            println!(
                "using input bwa index at: {}",
                linear_genome_index.display()
            );
        }

        let scenario = format!("{}/resources/scenarios/scenario.yaml", cargo_dir);

        //perform the alignment for paired end reads
        let _temp_aligned = NamedTempFile::new()?;

        //find sample name of one of the fastq files from the read pair
        let stem_of_sample_dir = &self.reads[0].file_stem().unwrap().to_str().unwrap();
        //get rid of any underscore (PE reads contain them)
        let splitted = stem_of_sample_dir.split('_').collect::<Vec<&str>>();
        let sample_name = splitted[0];

        //create the output file name in temp directory
        let file_aligned = temp_dir.path().join(format!("{}.bam", sample_name));
        println!("{}", file_aligned.display());

        //insert read_group info from the sample names
        let read_group = format!("@RG\\tID:{}\\tSM:{}", sample_name, sample_name);

        //Step-1: align reads to the bwa index
        let align = {
            Command::new("bwa")
                .arg("mem")
                .arg("-t")
                .arg("10")
                .arg("-R")
                .arg(&read_group)
                .arg(linear_genome_index)
                .arg(&self.reads[0])
                .arg(&self.reads[1])
                .arg("-o")
                .arg(&file_aligned)
                // .arg("2>")
                // .arg("log.txt")
                .status()
                .expect("failed to execute the alignment process")
        };
        println!("The alignment was exited with: {}", align);
        println!("{}", file_aligned.display());
        //sort the aligned reads by coordinate

        //create the output file name in temp directory
        let file_aligned_sorted: PathBuf =
            temp_dir.path().join(format!("{}_sorted.bam", sample_name));
        // let file_aligned_sorted: PathBuf = outdir.join(format!("{}_sorted.bam", sample_name));

        let sort = {
            Command::new("samtools")
                .arg("sort")
                .arg(file_aligned)
                .arg("-o")
                .arg(&file_aligned_sorted)
                .arg("-@")
                .arg(&self.threads)
                .arg("--write-index")
                .status()
                .expect("failed to execute the sorting process")
        };
        println!("The sorting was exited with: {}", sort);
        println!("{}", file_aligned_sorted.display());

        //Step-2: extract reads that map to HLA genes (classical and nonclassical class of genes)

        //before the extraction with samtools, check if the used genome has has ensembl or ucsc style chr namings and write regions to file
        let path_idxstats: PathBuf = temp_dir.path().join("stats.txt");
        // let path_idxstats = outdir.join("stats.txt");
        let mut file_idxstats = std::fs::File::create(path_idxstats.clone())?;

        let idxstats = {
            Command::new("samtools")
                .arg("idxstats")
                .arg(&file_aligned_sorted)
                .output()
                .expect("failed to execute idxstat process")
        };
        file_idxstats.write_all(&idxstats.stdout)?; //write with bam writer
        file_idxstats.flush()?;

        // Build the CSV reader and iterate over each record.
        let mut chr_naming = &"ensembl";
        let mut reader = ReaderBuilder::new()
            .delimiter(b'\t')
            .from_path(path_idxstats)?;

        // for result in reader.records() {
        //     // break after the first record, because one record is enough to see the chr naming style
        //     let record = result?;
        //     let record_string = record[0].to_string();
        //     // println!("{:?}", record_string);
        //     if record_string.starts_with("chr") {
        //         chr_naming = &"ucsc";
        //     }
        //     break;
        // }

        if let Some(result) = reader.records().next() {
            let record = result?;
            let record_string = record[0].to_string();
            // println!("{:?}", record_string);
            if record_string.starts_with("chr") {
                chr_naming = &"ucsc";
            }
        }

        println!("chr_naming format: {}", chr_naming);
        let path_to_regions = outdir.join("regions.bed");
        let mut regions_file = std::fs::File::create(&path_to_regions)?;
        if chr_naming == &"ucsc" {
            let regions_ensembl = "\
chr6\t32659467\t32668383
chr6\t32577902\t32589848
chr6\t32628179\t32647062
chr6\t31268749\t31272130
chr6\t30489509\t30494194
chr6\t29826967\t29831125
chr6\t29722775\t29738528
chr6\t29887752\t29890482
chr6\t29941260\t29949572
chr6\t31353872\t31367067";
            regions_file.write_all(regions_ensembl.as_bytes())?;
        } else if chr_naming == &"ensembl" {
            let regions_ucsc = "\
6\t32659467\t32668383
6\t32577902\t32589848
6\t32628179\t32647062
6\t31268749\t31272130
6\t30489509\t30494194
6\t29826967\t29831125
6\t29722775\t29738528
6\t29887752\t29890482
6\t29941260\t29949572
6\t31353872\t31367067
            ";
            regions_file.write_all(regions_ucsc.as_bytes())?;
        }
        regions_file.flush()?;

        //create the output file name in temp directory
        let file_extracted = temp_dir
            .path()
            .join(format!("{}_extracted.bam", sample_name));
        // let file_extracted = outdir.join(format!("{}_extracted.bam", sample_name));
        // let regions = format!("{}/resources/regions.bed", cargo_dir);

        let extract = {
            Command::new("samtools")
                .arg("view")
                .arg(file_aligned_sorted)
                .arg("-L")
                .arg(path_to_regions)
                .arg("--write-index") //??
                .arg("-o")
                .arg(&file_extracted)
                .status()
                .expect("failed to execute the extracting process")
        };
        println!("The extraction was exited with: {}", extract);

        //convert the alignment file to fq

        //create the output file name in temp directory
        let temp_extracted_fq_1 = temp_dir.path().join(format!("{}_1.fastq", sample_name));
        let temp_extracted_fq_2 = temp_dir.path().join(format!("{}_2.fastq", sample_name));

        let bam_to_fq = {
            Command::new("samtools")
                .arg("fastq")
                .arg(file_extracted)
                .arg("-n") //-n for fastq
                .arg("-1")
                .arg(&temp_extracted_fq_1)
                .arg("-2")
                .arg(&temp_extracted_fq_2)
                .status()
                .expect("failed to execute the extracting process")
        };
        println!("Conversion from BAM to fq was exited with: {}", bam_to_fq);

        //Step-3: map extracted reads to the pangenome with vg giraffe

        //path to the index directory
        // let vg_index = "resources/hprc-v1.0-mc-grch38.xg";

        //create the output file name in temp directory
        let file_aligned_pangenome = outdir.join(format!("{}_vg.bam", sample_name));

        let align_pangenome = {
            Command::new("vg")
                .arg("giraffe")
                .arg("-x")
                .arg(self.vg_index.clone())
                .arg("-f")
                .arg(temp_extracted_fq_1)
                .arg("-f")
                .arg(temp_extracted_fq_2)
                .arg("--output-format")
                .arg("BAM")
                .arg("-t")
                .arg(&self.threads)
                .stdout(Stdio::piped())
                .spawn()
                .expect("failed to execute the vg giraffe process")
        };
        println!(
            "Alignment to pangenome was exited with: {:?}",
            align_pangenome
        );

        //write bam to file (buffered)
        // let mut vg_bam = std::fs::File::create(&file_aligned_pangenome)?;
        // let mut f = std::fs::File::open(&file_aligned_pangenome).unwrap();
        // let mut f = std::io::BufWriter::new(f);
        // {
        //     let stdout = align_pangenome.stdout;
        //     let stdout_reader = std::io::BufReader::new(stdout);
        //     let stdout_lines = stdout_reader.bytes();

        //     for line in stdout_lines {
        //         f.write(&[line.unwrap()]);
        //     }
        // }
        // align_pangenome.wait().unwrap();
        // f.flush()?;

        // let output_align = align_pangenome.stdout.expect("failed to wait on aligning to pangenome");

        let output = align_pangenome
            .wait_with_output()
            .expect("Failed to read stdout");

        let mut vg_bam = std::fs::File::create(file_aligned_pangenome.clone())?;
        vg_bam.write_all(&output.stdout)?; //write with bam writer
        vg_bam.flush()?;

        //sort the resulting vg aligned file
        let file_vg_aligned_sorted = temp_dir
            .path()
            .join(format!("{}_vg_sorted.bam", sample_name));

        let vg_sort = {
            Command::new("samtools")
                .arg("sort")
                .arg(&file_aligned_pangenome)
                .arg("-o")
                .arg(&file_vg_aligned_sorted)
                .arg("-@")
                .arg(&self.threads)
                .arg("--write-index")
                .status()
                .expect("failed to execute the sorting process")
        };
        println!("The sorting was exited with: {}", vg_sort);
        println!("{}", file_vg_aligned_sorted.display());

        //modify the header for chromosome names to be compatible with the reference genome that we acquire from ensembl

        //prepare the temporary file path for the reheadered bam output
        let file_reheadered = temp_dir
            .path()
            .join(format!("{}_reheadered.bam", sample_name));

        println!("{}", file_reheadered.display());

        //in Rust, piping cannot be done via "|" but instead in the following way:

        //get the header
        let samtools_view_child = Command::new("samtools")
            .arg("view") // `samtools view` command...
            .arg("-H") // of which we will pipe the output.
            .arg(&file_vg_aligned_sorted) //Once configured, we actually spawn the command...
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        //replace the 'GRCh38.chr' with '' or "chr" prefices depending on the genome reference chr naming style
        let mut regex = &"";
        if chr_naming == &"ucsc" {
            regex = &"s/GRCh38.//g";
        } else if chr_naming == &"ensembl" {
            regex = &"s/GRCh38.chr//g";
        }
        println!("regex for reheader: {}", regex);
        let sed_child_one = Command::new("sed")
            .arg(regex)
            .stdin(Stdio::from(samtools_view_child.stdout.unwrap())) // Pipe through.
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        //then, reheader the header of the input bam
        let reheader_child_two = Command::new("samtools")
            .arg("reheader")
            .arg("-")
            .stdin(sed_child_one.stdout.unwrap())
            .arg(file_vg_aligned_sorted)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        //write the reheadered bam to file
        let output = reheader_child_two
            .wait_with_output()
            .expect("failed to wait on child");
        let mut f = std::fs::File::create(file_reheadered.clone())?;
        f.write_all(&output.stdout)?;

        //index the resulting bam file
        let samtools_index = {
            Command::new("samtools")
                .arg("index")
                .arg(&file_reheadered)
                .status()
                .unwrap()
        };

        println!("The indexing was exited with: {}", samtools_index);

        //finally, extract only strandard chromosomes
        let final_bam = outdir.join(format!("{}_processed.bam", sample_name));
        println!("{}", final_bam.display());

        //construct chromosome names according to the genome reference chr naming style
        let mut chromosomes = vec![];
        if chr_naming == &"ucsc" {
            chromosomes = vec![
                "chr1", "chr2", "chr3", "chr4", "chr5", "chr6", "chr7", "chr8", "chr9", "chr10",
                "chr11", "chr12", "chr13", "chr14", "chr15", "chr16", "chr17", "chr18", "chr19",
                "chr20", "chr21", "chr22", "chrX", "chrY", "chrM",
            ]
        } else if chr_naming == &"ensembl" {
            chromosomes = vec![
                "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15",
                "16", "17", "18", "19", "20", "21", "22", "X", "Y", "M",
            ]
        }
        println!("chromosomes to extract: {:?}", chromosomes);

        let samtools_extract = {
            Command::new("samtools")
                .arg("view")
                .arg(&file_reheadered)
                .args(chromosomes)
                .arg("-o")
                .arg(&final_bam)
                .arg("-@")
                .arg(&self.threads)
                .arg("--write-index")
                .status()
                .expect("failed to execute the sorting process")
        };

        //write the final bam to file
        println!(
            "The extractiong of standard chromosomes was exited with: {}",
            samtools_extract
        );

        //varlociraptor preprocess and call

        //preprocess
        //create the output file name
        let varlociraptor_prep_dir = outdir.join(format!("{}_obs.bcf", sample_name));
        println!(
            "varlociraptor_prep_dir: {}",
            varlociraptor_prep_dir.display()
        );

        let varlociraptor_prep = {
            Command::new("varlociraptor")
                .arg("preprocess")
                .arg("variants")
                .arg("--report-fragment-ids")
                .arg("--omit-mapq-adjustment")
                .arg("--atomic-candidate-variants")
                .arg("--candidates")
                .arg(&self.haplotype_variants)
                .arg(&self.genome)
                .arg("--bam")
                .arg(&final_bam)
                .arg("--output")
                .arg(&varlociraptor_prep_dir)
                .status()
                .expect("failed to execute the varlociraptor preprocessing")
        };
        println!(
            "The varlociraptor preprocessing was exited with: {}",
            varlociraptor_prep
        );

        //call
        // "varlociraptor call variants --omit-strand-bias --omit-read-position-bias --omit-read-orientation-bias --omit-softclip-bias --omit-homopolymer-artifact-detection --omit-alt-locus-bias generic --obs sample={input.obs} " ##varlociraptor v5.3.0
        // "--scenario {input.scenario} > {output} 2> {log}"
        //create the output file name
        let varlociraptor_call_dir = outdir.join(format!("{}.bcf", sample_name));
        println!(
            "varlociraptor_call_dir: {}",
            varlociraptor_call_dir.display()
        );

        //scenario
        println!(
            "{}",
            format!("sample={}", &varlociraptor_prep_dir.display())
        );

        let varlociraptor_call = {
            Command::new("varlociraptor")
                .arg("call")
                .arg("variants")
                .arg("--omit-strand-bias")
                .arg("--omit-read-position-bias")
                .arg("--omit-read-orientation-bias")
                .arg("--omit-softclip-bias")
                .arg("--omit-homopolymer-artifact-detection")
                .arg("--omit-alt-locus-bias")
                .arg("generic")
                .arg("--obs")
                .arg(format!("sample={}", varlociraptor_prep_dir.display()))
                .arg("--scenario")
                .arg(&scenario)
                .stdout(Stdio::piped())
                .spawn()
                .expect("failed to execute the varlociraptor calling process")
        };
        println!(
            "The varlociraptor calling was exited with: {:?}",
            varlociraptor_call
        );

        let output = varlociraptor_call
            .wait_with_output()
            .expect("Varlociraptor: Failed to read stdout");
        let mut called_file = std::fs::File::create(&varlociraptor_call_dir)?;
        called_file.write_all(&output.stdout)?; //write with bam writer
        called_file.flush()?;
        // close the file handle of the named temporary files
        temp_dir.close()?;

        Ok(())
    }
}
