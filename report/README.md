# Sheesee Project Report - LaTeX Documentation

This directory contains the complete LaTeX source files for the Sheesee Image Processing and Google Sheets Integration System project report.

## Structure

```
report/
├── main.tex                    # Main LaTeX file (compile this)
├── chapters/
│   ├── titlepage.tex          # Title page
│   ├── acknowledgement.tex    # Acknowledgement section
│   ├── abstract.tex           # Abstract
│   ├── acronyms.tex           # List of acronyms
│   ├── chapter1.tex           # Introduction
│   ├── chapter2.tex           # System Analysis
│   ├── chapter3.tex           # System Implementation
│   ├── chapter4.tex           # Results
│   └── chapter5.tex           # Conclusion and Recommendations
└── README.md                  # This file
```

## Compilation Instructions

### Prerequisites

1. **LaTeX Distribution**: Install one of the following:
   - **Windows**: MiKTeX (https://miktex.org/) or TeX Live
   - **Linux**: `sudo apt-get install texlive-full`
   - **macOS**: MacTeX (https://www.tug.org/mactex/)

2. **PDF Viewer**: Any PDF viewer (Adobe Reader, SumatraPDF, Evince, etc.)

### Compiling the Document

#### Using Command Line

Navigate to the `report` directory and run:

```bash
pdflatex main.tex
pdflatex main.tex  # Run twice to resolve references
```

Or use the complete build sequence:

```bash
pdflatex main.tex
pdflatex main.tex
pdflatex main.tex  # Third run for table of contents, lists, etc.
```

#### Using LaTeX Editors

**Overleaf (Online)**:

1. Create a new project
2. Upload all `.tex` files maintaining the folder structure
3. Set `main.tex` as the main document
4. Click "Recompile"

**TeXstudio/TeXworks (Desktop)**:

1. Open `main.tex`
2. Click "Build & View" or press F5
3. The PDF will be generated automatically

**VS Code with LaTeX Workshop**:

1. Install the "LaTeX Workshop" extension
2. Open `main.tex`
3. Save the file (Ctrl+S) - it will compile automatically
4. Or use Ctrl+Alt+B to build manually

## Customization Guide

### 1. Update Title Page

Edit `chapters/titlepage.tex`:

```latex
% Replace these placeholders:
- [University Logo] -> Add your logo file
- Student Name 1/2 -> Your names
- Roll No: XXXXX -> Your roll numbers
- Dr./Prof. [Instructor Name] -> Actual instructor name
- [University Name] -> Your university
- Subject Code: COMP342 -> Actual course code
```

### 2. Add Images

The report includes placeholders for figures. Create an `images/` directory in the `report/` folder:

```bash
mkdir images
```

Add the following images (or create placeholders):

**Required Images:**

- `logo.png` - University/department logo (for title page)
- `block_diagram.png` - System architecture diagram
- `oauth_flow.png` - OAuth sequence diagram
- `cli_startup.png` - Terminal screenshot at startup
- `oauth_url.png` - OAuth URL generation screenshot
- `google_consent.png` - Google authorization page
- `auth_complete.png` - Authentication success message
- `input_sample.jpg` - Sample input image
- `resized_image.png` - Resized 400x300 image
- `before_convolution.png` - Image before processing
- `after_convolution.png` - Image after convolution
- `sheets_full_view.png` - Full Google Sheets render
- `sheets_zoomed.png` - Zoomed cell view
- `cell_properties.png` - Cell formatting properties

**Creating Placeholder Images:**

You can use ImageMagick to create simple placeholders:

```bash
# Create a placeholder image with text
convert -size 800x600 xc:lightgray -pointsize 40 -draw "text 250,300 'Block Diagram'" block_diagram.png
```

Or use online tools like:

- https://placeholder.com/
- https://via.placeholder.com/800x600.png?text=Your+Image+Here

### 3. Update Personal Information

**Acknowledgement** (`chapters/acknowledgement.tex`):

- Replace `[Instructor Name]` with actual name
- Replace `[University Name]` with your institution
- Replace `[Student Name 1]` and `[Student Name 2]` with your names

**Abstract** (`chapters/abstract.tex`):

- Review and adjust if needed based on your specific implementation

### 4. Adjust Technical Details

If your implementation differs, update:

**Chapter 2** (`chapters/chapter2.tex`):

- Software/hardware requirements tables
- Subject code and course name

**Chapter 3** (`chapters/chapter3.tex`):

- Code snippets to match your actual implementation
- File paths and structure

**Chapter 4** (`chapters/chapter4.tex`):

- Performance measurements with actual data
- Test results based on your testing

## Common Issues and Solutions

### Issue 1: Missing Packages

**Error**: `! LaTeX Error: File 'package.sty' not found`

**Solution**: Install the missing package:

- **MiKTeX**: Packages install automatically on first use
- **TeX Live**: `sudo tlmgr install <package-name>`

### Issue 2: Images Not Found

**Error**: `! LaTeX Error: File 'image.png' not found`

**Solution**:

- Create the `images/` directory if it doesn't exist
- Ensure image files are in the correct location
- Check file extensions match (case-sensitive on Linux/Mac)

### Issue 3: References Not Resolved

**Error**: "??" appears instead of figure/table numbers

**Solution**: Compile the document multiple times (at least twice)

### Issue 4: Table of Contents Empty

**Solution**: Run `pdflatex` at least twice to generate TOC

## Advanced Customization

### Adding Citations

If you want to add references:

1. Create a `references.bib` file:

```bibtex
@article{example2024,
    author = "Author Name",
    title = "Paper Title",
    journal = "Journal Name",
    year = "2024"
}
```

2. Uncomment in `main.tex`:

```latex
\bibliography{references}  % Remove the % at the start
```

3. Compile with:

```bash
pdflatex main.tex
bibtex main
pdflatex main.tex
pdflatex main.tex
```

### Changing Fonts

Add to the preamble in `main.tex`:

```latex
\usepackage{times}  % Times New Roman
% or
\usepackage{charter}  % Charter font
```

### Adjusting Margins

Modify in `main.tex`:

```latex
\usepackage[margin=1in]{geometry}  % Change 1in to desired margin
```

### Line Spacing

Already set to 1.5 spacing. To change:

```latex
\onehalfspacing  % 1.5 spacing (current)
\doublespacing   % 2.0 spacing
\singlespacing   % 1.0 spacing
```

## Report Statistics

- **Total Pages**: Approximately 50-60 pages (with images)
- **Chapters**: 5 main chapters
- **Figures**: 14 placeholders
- **Tables**: 10 tables
- **Word Count**: ~12,000 words

## Quality Checklist

Before final submission:

- [ ] All placeholder names replaced (instructor, university, students)
- [ ] All images added or appropriate placeholders included
- [ ] Compiled successfully without errors
- [ ] Table of Contents generated correctly
- [ ] All figures and tables numbered correctly
- [ ] Page numbers present and correct
- [ ] No "??" references remaining
- [ ] Spell-checked and proofread
- [ ] Code listings match actual implementation
- [ ] Performance data reflects actual measurements

## Output

After successful compilation, you'll have:

- `main.pdf` - Your final report (50-60 pages)
- `main.aux`, `main.log`, `main.toc`, etc. - Auxiliary files (can be deleted)

## Support

If you encounter issues:

1. Check the `.log` file for detailed error messages
2. Search for the error message online (LaTeX errors are well-documented)
3. Ensure all required packages are installed
4. Verify file paths and structure match this README

## License

This LaTeX template is provided as-is for educational purposes. Feel free to modify and adapt for your needs.

---

**Generated for**: Sheesee Image Processing Project
**Date**: January 2026
**LaTeX Version**: pdfLaTeX or XeLaTeX compatible
