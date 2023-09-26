#!/bin/bash -uex
if [ -f /etc/lsb-release ]; then
    DISTRO=ubuntu
elif [ -f /etc/arch-release ]; then
    DISTRO=archlinux
    INSTALL="yay -S"
else
    echo "Unsupported Linux distribution."
    exit 1
fi

Check the Linux distribution
if [ "ubuntu" = $DISTRO ]; then
    # Ubuntu
    echo "Detected Ubuntu. Installing Anaconda..."
    EXPECTED_HASH=75967bc2113d9e336e670e1e557c9198d8b98e59fb9adb82cbe0e71ce5f7c2db
    wget -O AnacondaInstaller.sh "https://repo.anaconda.com/archive/Anaconda3-2023.07-2-Linux-x86_64.sh"
    ACTUAL_HASH=$(sha256sum AnacondaInstaller.sh | awk '{print $1}')
    if [ "$ACTUAL_HASH" != "$EXPECTED_HASH" ]; then
        echo "Hash mismatch for anaconda instaler"
        exit 1
    fi
    bash AnacondaInstaller.sh
    rm AnacondaInstaller.sh
    echo "Anaconda installed successfully on Ubuntu."
elif [ "archlinux" = $DISTRO ]; then
    # Arch Linux
    echo "Detected Arch Linux. Installing Anaconda..."
    $INSTALL anaconda
    echo "Anaconda installed successfully on Arch Linux."
fi
echo "ensure [ -f /opt/anaconda/etc/profile.d/conda.sh ] && source /opt/anaconda/etc/profile.d/conda.sh" >> ~/.bashrc
source ~/.bashrc

conda config --add channels bioconda
conda config --add channels conda-forge
conda create -n bio -y python=3.10
conda install -n bio -c biopython blast

if [ "archlinux" = $DISTRO ]; then
    # $INSTALL bedtools
    $INSTALL bcftools bioawk bwa bowtie2 csvtk datamash-git emboss fastqc fastp freebayes hisat2 mafft parallel perl-text-csv samtools seqtk seqkit-bin snpeff subread trimmomatic ucsc-bedgraphtobigwig wget
fi
