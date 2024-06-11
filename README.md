<div align='center'>
    <img src="https://i.imgur.com/ZZ9a1DU.png"/>
</div>

<p align='center'><b>Unleash your knowledge.</b></p>

<p align='center'>
	<a href='https://github.com/Scibun/Scibun/actions/workflows/rust.yml'><img src='https://github.com/Scibun/Scibun/actions/workflows/rust.yml/badge.svg'/></a>
</p>

## Documentation

For more help and document, see our documentation:

- [How to build](https://scibun.github.io/ScimonDocs/build.html)
- [Basic usage](https://scibun.github.io/ScimonDocs/basic-usage.html)
- [Flags](https://scibun.github.io/ScimonDocs/flags.html)
- [Downloads Block](https://scibun.github.io/ScimonDocs/download-block.html)
- [Readme Block](https://scibun.github.io/ScimonDocs/readme-block.html)
- [Open links](https://scibun.github.io/ScimonDocs/open-links.html)
- [Checksum and Checksum Validate](https://scibun.github.io/ScimonDocs/checsum.html)
- [Directives and Comments](https://scibun.github.io/ScimonDocs/directives.html)
- [Markdown render](https://scibun.github.io/ScimonDocs/markdown-render.html)
- [Scrape](https://scibun.github.io/ScimonDocs/scrape.html)
- [Providers](https://scibun.github.io/ScimonDocs/providers.html)
- [Scimon.yml file](https://scibun.github.io/ScimonDocs/scimon.yml-file.html)
- [.env file](https://scibun.github.io/ScimonDocs/env-file.html)
- [External Resources Usage](https://scibun.github.io/ScimonDocs/external-resources.html)

## Example of code and execute

```monset
path = "downloads/"
open = "https://scibun.com"

readme = "https://example.com/readme-example.md"
checksum = "https://example.com/scimon.sha256"
checksum.unmatch = "keep"

downloads {
    https://example.com/file1.pdf !ignore
    https://example.com/file2.pdf
    https://example.com/file3.pdf !ignore
    https://example.com/file4.pdf
}
```

Run the command:

```bash
scimon -r scimon.mon
```
