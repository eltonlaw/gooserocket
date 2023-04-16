# spherical cow

spherical-cow is a computational bioinformatics research platform for running experiments on AWS.

## Design goals

These are not set in stone:

1. Keeping costs low: AWS spot instances will be used, so things will be built such that they are interruption tolerant. Things will be cleaned up as soon as they're finished. Data will mostly be stored in S3.
    1. TODO: see if S3 IA can fit with spot instances
    2. TODO: Good way to infer optimal instance size? Could use historical data, would need to save that metadata
2. Generic primitives: A rust function, which does something to some input and outputs something, run on some image, will serve as the main compute abstraction.
3. User interface will be half CLI, half Jupyter notebook: Infra and experiments managed by CLI, if need to dig into results, jupyter notebook can call those same tools. Jupyter notebook will also be used to process, visualize and analyze experimental results.

## Aspirations

- [ ] Fn to spin up an ec2 instance that uses alphafold to predict protein structure given amino acid sequence.
- [ ] Read in compounds from PubChem and run AutoDock Vina on them with some target protein in a distributed fashion
- [ ] Run similarity analysis using rxrx3
- [ ] Build up a database of generic data.

## To Do

- scow-data: Lib code for getting & managing heterogeneous data
    * [ ] API call to get amino acid sequence from uniprot given id, ex. P0DTC2
    * [ ] API call to get protein structure from pdb given the pdb id, ex. 6VXX
    * [ ] Restructure and index alphafold data
    * [ ] Investigate better compression/decompression for use cases
    * [ ] Export to .pdb
    * [ ] Export to .fasta
- scow-bio: Algorithms
    * [ ] AutoDock Vina scoring
    * [ ] Poisson-Boltzmann model
- scow-engine: Launches experiments
    * [ ] Launch a jupyter notebook with required deps
    * [ ] Package some entrypoint into a spot instance fleet and schedule
- scow-infra:
    * [ ] Parallelized s3:GetObject from s3
    * [ ] Parallelized s3:PutObject to s3
    * [ ] create cloudformation stack
    * [ ] update cloudformation stack
- scow-cli: CLI for interacting with resources
    * [ ] `./scow-cli deploy <target>`
    * [ ] `./scow-cli infra datasources ls`
    * [ ] `./scow-cli experiments ls`
    * [ ] `./scow-cli jobs ls`
    * [ ] `./scow-cli system prune`
- scow-tracing: utils for tracing
    * [ ] export to honeycomb.io for initial experiments monitoring
