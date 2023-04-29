# gooserocket

gooserocket is a computational bioinformatics research platform for running experiments on AWS.

## Design goals

These are not set in stone:

1. Keeping costs low: AWS spot instances will be used, so things will be built such that they are interruption tolerant.
    1. No idle resources. Only select things in S3 will be persisted.
    2. All jobs run through spot instances, compute set up to be idempotent and interruptible.
    3. Minimize egress from S3/ECR, keep everything in same region, HA is unneccessary. At \$0.09/GB, grabbing terabyte level data from S3 gets super expensive 
2. User interface will be half CLI, half Jupyter notebook: Infra and experiments managed by CLI, if need to dig into results, jupyter notebook can call those same tools. Jupyter notebook will also be used to process, visualize and analyze experimental results.

## Aspirations

- [ ] Fn to spin up an ec2 instance that uses alphafold to predict protein structure given amino acid sequence.
- [ ] Compare protein sequence to other protein sequences (FASTA / PSI-BLAST for biologically significant match)
- [ ] Read in compounds from PubChem and run AutoDock Vina on them with some target protein in a distributed fashion
- [ ] Run similarity analysis using rxrx3
- [ ] Build up database of subset of public data with focus on DNA/Protein sequences, macromolecule structures and the results of genomics experiments


## To Do

- gr-data: Lib code for getting & managing heterogeneous data
    * [ ] API call to get amino acid sequence from uniprot given id, ex. P0DTC2
    * [ ] API call to get protein structure from pdb given the pdb id, ex. 6VXX
    * [ ] Accessing genbank's nucleic acid sequences
    * [ ] Restructure and index alphafold data
    * [ ] Investigate better compression/decompression for use cases
    * [ ] Export to .pdb
    * [ ] Export to .fasta
- gr-bio: Algorithms
    * [ ] AutoDock Vina scoring
    * [ ] Poisson-Boltzmann model
- gr-engine: Launches experiments
    * [ ] Launch a jupyter notebook with required deps
    * [ ] Package some entrypoint into a spot instance fleet and schedule
- gr-infra:
    * [ ] Parallelized s3:GetObject from s3
    * [ ] Parallelized s3:PutObject to s3
    * [ ] create cloudformation stack
    * [ ] update cloudformation stack
- gr-cli: CLI for interacting with resources
    * [ ] `./gr-cli deploy <target>`
    * [ ] `./gr-cli infra datasources ls`
    * [ ] `./gr-cli experiments ls`
    * [ ] `./gr-cli jobs ls`
    * [ ] `./gr-cli system prune`
    * [ ] `./gr-cli shutdown all`
- gr-tracing: utils for tracing
    * [ ] export to honeycomb.io for initial experiments monitoring

## Setup

Not all required AWS resources are in CF such that you can deploy the entire platform cold

- security group
- ssh key-pair
- create billing reports sent to bucket
