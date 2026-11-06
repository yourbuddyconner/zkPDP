"""Write paper/draft.md from paper/main.tex, so the two never disagree.

main.tex is the source. This runs pandoc, then fixes what pandoc loses: the
title and abstract, section numbers, experiment tags, section references,
paragraph headings and the two-level header of Table 2.
Run from any directory: python3 paper/to_markdown.py
"""

import pathlib
import re
import subprocess

HERE = pathlib.Path(__file__).resolve().parent
TEX = HERE / "main.tex"
OUT = HERE / "draft.md"

FORMAT = "markdown" + "".join(
    f"-{x}"
    for x in (
        "raw_tex raw_attribute bracketed_spans native_divs native_spans "
        "header_attributes simple_tables multiline_tables grid_tables fenced_divs"
    ).split()
)


def main():
    tex = TEX.read_text()
    md = subprocess.run(
        ["pandoc", "-f", "latex", "-t", FORMAT, "--wrap=none", str(TEX)],
        check=True, capture_output=True, text=True,
    ).stdout

    # Experiment tags and section references.
    md = re.sub(r'<span class="sans-serif">\\\[([^\]]+)\\\]</span>', r"[\1]", md)
    md = re.sub(r'<span style="color: [^"]*">(.*?)</span>', r"\1", md)
    md = re.sub(r"\\\[(E[0-9a-z]+|TODO: [^\]]*)\\\]", r"[\1]", md)
    md = re.sub(r'\[([0-9A-Z.]+)\]\(#[^)]*\)\{reference-type="ref" reference="[^"]*"\}', r"\1", md)

    md = md.replace("$\\approx$<!-- -->", "≈").replace("$\\approx$", "≈")
    md = re.sub(r"(\d)--(\d)", "\\1–\\2", md)

    # Paragraph headings become bold lead-ins, as in the PDF.
    md = re.sub(r"^#### (.+)\n\n", r"**\1** ", md, flags=re.M)

    # Number sections and subsections the way LaTeX does, appendix as A.
    out, sec, sub, appendix = [], 0, 0, False
    unnumbered = {"Disclosure"}
    for line in md.splitlines():
        m = re.match(r"^(#{1,2}) (.+)$", line)
        if m and m.group(2) not in unnumbered:
            if m.group(2) == "Artifact":
                appendix, sec = True, 0
            if len(m.group(1)) == 1:
                sec, sub = sec + 1, 0
                label = chr(64 + sec) if appendix else str(sec)
                line = f"## {label}. {m.group(2)}"
            else:
                sub += 1
                line = f"### {sec}.{sub} {m.group(2)}"
        elif m:
            line = f"## {m.group(2)}"
        out.append(line)
    md = "\n".join(out) + "\n"

    # Table 2 has a two-level header that pipe tables cannot hold.
    md = re.sub(
        r"\|[^\n]*Noir[^\n]*GordieHowe[^\n]*\|\n(\|[:\- |]+\|)\n\| 3-5\(l\)6-9 Set[^\n]*\|\n",
        lambda m: "| Set | Requests | Noir gates | Noir prove | Noir memory | GordieHowe prove "
        "| GordieHowe verify | GordieHowe proof | GordieHowe compile |\n" + m.group(1) + "\n",
        md,
    )

    # Captions: pandoc puts each table's caption after it as plain text.
    md = re.sub(r"^(:::.*|</?div[^>]*>)\n", "", md, flags=re.M)
    md = re.sub(r"^: (.+)$", r"*\1*", md, flags=re.M)

    title = re.search(r"\\title\{(.+?)\}\n\\author", tex, re.S).group(1)
    title = re.sub(r"\\\\(\[[^\]]*\])?\s*", " ", title).replace("\n", " ")
    title = re.sub(r"\\textbf\{([^}]*)\}", r"\1", title).replace("\\Large", "")
    title = re.sub(r"\s+", " ", title).strip()
    abstract = re.search(r"\\begin\{abstract\}\n(.+?)\n\\end\{abstract\}", tex, re.S).group(1)
    abstract = abstract.replace("\\noindent", "").replace("~", " ").replace("\\", "").strip()
    head = f"# {title}\n\nConner Swann. Preprint, 6 November 2026.\n\n"
    md = head + "## Abstract\n\n" + abstract + "\n\n" + md.lstrip()
    OUT.write_text(md)
    print(f"wrote {OUT.relative_to(HERE.parent)}, {len(md.split())} words")


if __name__ == "__main__":
    main()
