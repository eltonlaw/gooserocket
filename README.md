# gooserocket

gooserocket is a computational bioinformatics research platform for running experiments on AWS. The target is to provide a platform that provides a cheap way to:

1. Schedule distributed batch jobs for grabbing/searching/categorizing/featurizing data and storing in S3.
2. Provide a library of primitives to work with the various public databases and biological data formats
3. Spin up jupyter notebooks with access to pre processed data. Run ML training/prediction/analytics.

## Design goals

(Not set in stone)

1. Keeping costs low: AWS spot instances will be used, so things will be built such that they are interruption tolerant.
    1. No idle resources. Only select things in S3 will be persisted.
    2. All jobs run through spot instances, compute set up to be idempotent and interruptible.
    3. Minimize egress from S3/ECR, keep everything in same region, HA is unneccessary. At \$0.09/GB, grabbing terabyte level data from S3 gets super expensive 
2. Most of the scientific work will be done in a jupyter notebook. Heavy processing will handled by an orchestrator which takes in requests, spins up the necessary infra and responds immediately with a response id. From the jupyter nb, you'll ask about the response id until its succeeded/failed at which point there'll be a file in s3.

## Aspirations

- [ ] Use alphafold to predict protein structure given amino acid sequence.
- [ ] Compare protein sequence to other protein sequences (FASTA / PSI-BLAST for biologically significant match)
- [ ] Read in compounds from PubChem and run AutoDock Vina on them with some target protein in a distributed fashion
- [ ] Run similarity analysis using rxrx3
- [ ] Build up database of results of genomics experiments from public data
- [ ] Creating a blast database in S3 and running blast on reference genome to see alignment

## To Do

- gr-data: Lib code for getting & managing heterogeneous data
    * [ ] Fn to get amino acid sequence from uniprot given id, ex. P0DTC2
    * [ ] Fn to get protein structure from pdb given the pdb id, ex. 6VXX
    * [ ] Fn to access genbank's nucleic acid sequences
    * [ ] Investigate data specific compression to minimize on-disk size.
    * [ ] Import/export SMILES - [spec](http://opensmiles.org/opensmiles.html)
    * [ ] Import/export .pdb - [link](https://www.cgl.ucsf.edu/chimera/docs/UsersGuide/tutorials/pdbintro.html)
    * [ ] Import/export .fasta - [wiki](https://en.wikipedia.org/wiki/FASTA_format)
- gr-bio: Algorithms
    * [ ] AutoDock Vina scoring
    * [ ] Implement everything in https://rosalind.info/problems/list-view/
- gr-engine: Launches experiments
    * [x] Launch a jupyter notebook with required deps
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

## Infra

Currently just cloudformation stacks deployed via aws api calls by rust sdk using [these hardcoded CF templates](https://github.com/eltonlaw/gooserocket/tree/main/assets)

NOTE: Not all required AWS resources are in CF such that you can deploy the entire platform cold:  security group, ssh key-pair, create billing reports sent to bucket
