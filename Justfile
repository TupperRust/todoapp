memory := ".memory"

help:
    @just --list

add name:
    @cargo run -- create {{ name }}
    @cat {{ memory }}

insert name task:
    @cargo run -- insert {{ name }} {{ task }}
    @cat {{ memory }}

done name task:
    @cargo run -- done {{ name }} {{ task }}
    @cat {{ memory }}
