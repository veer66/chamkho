# Chamkho
Lao/Thai word segmentation library in Rust

# Usage

## As command line

    echo "กากกา" | wordcut 

### External dictionary

    echo "กากกา" | wordcut -d <path to dictionary>

### Specific language

    echo "ພາສາລາວມີ" | wordcut -l lao
