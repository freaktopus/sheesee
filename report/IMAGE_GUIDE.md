# Image Placeholder Guide

This document explains how to create placeholder images for the Sheesee report while you prepare actual screenshots and diagrams.

## Quick Method: Online Placeholders

Visit these websites and download placeholder images:

### 1. Via Placeholder

URL: `https://via.placeholder.com/`

**Required Images:**

```
Title Page:
https://via.placeholder.com/300x300.png?text=University+Logo
-> Save as: logo.png

Chapter 2:
https://via.placeholder.com/1200x800.png?text=System+Block+Diagram
-> Save as: block_diagram.png

Chapter 3:
https://via.placeholder.com/1000x600.png?text=OAuth+Flow+Diagram
-> Save as: oauth_flow.png

Chapter 4:
https://via.placeholder.com/1200x800.png?text=CLI+Startup+Screen
-> Save as: cli_startup.png

https://via.placeholder.com/1200x800.png?text=OAuth+URL+Display
-> Save as: oauth_url.png

https://via.placeholder.com/1200x800.png?text=Google+Consent+Page
-> Save as: google_consent.png

https://via.placeholder.com/1200x800.png?text=Auth+Complete
-> Save as: auth_complete.png

https://via.placeholder.com/800x600.png?text=Input+Image
-> Save as: input_sample.jpg

https://via.placeholder.com/400x300.png?text=Resized+400x300
-> Save as: resized_image.png

https://via.placeholder.com/400x300.png?text=Before+Convolution
-> Save as: before_convolution.png

https://via.placeholder.com/400x300.png?text=After+Convolution
-> Save as: after_convolution.png

https://via.placeholder.com/1400x1000.png?text=Google+Sheets+Full+View
-> Save as: sheets_full_view.png

https://via.placeholder.com/800x600.png?text=Sheets+Zoomed+View
-> Save as: sheets_zoomed.png

https://via.placeholder.com/600x400.png?text=Cell+Properties
-> Save as: cell_properties.png
```

## Method 2: Python Script

Create a Python script to generate all placeholders:

```python
# generate_placeholders.py
from PIL import Image, ImageDraw, ImageFont

def create_placeholder(width, height, text, filename):
    """Create a placeholder image with text"""
    # Create image with light gray background
    img = Image.new('RGB', (width, height), color='#E0E0E0')
    draw = ImageDraw.Draw(img)

    # Try to use a nice font, fall back to default
    try:
        font = ImageFont.truetype("arial.ttf", 40)
    except:
        font = ImageFont.load_default()

    # Calculate text position (centered)
    bbox = draw.textbbox((0, 0), text, font=font)
    text_width = bbox[2] - bbox[0]
    text_height = bbox[3] - bbox[1]

    position = ((width - text_width) // 2, (height - text_height) // 2)

    # Draw text
    draw.text(position, text, fill='#666666', font=font)

    # Save image
    img.save(filename)
    print(f"Created: {filename}")

# Create all placeholders
placeholders = [
    (300, 300, "University Logo", "logo.png"),
    (1200, 800, "System Block Diagram", "block_diagram.png"),
    (1000, 600, "OAuth Flow Diagram", "oauth_flow.png"),
    (1200, 800, "CLI Startup Screen", "cli_startup.png"),
    (1200, 800, "OAuth URL Display", "oauth_url.png"),
    (1200, 800, "Google Consent Page", "google_consent.png"),
    (1200, 800, "Authentication Complete", "auth_complete.png"),
    (800, 600, "Sample Input Image", "input_sample.jpg"),
    (400, 300, "Resized 400x300", "resized_image.png"),
    (400, 300, "Before Convolution", "before_convolution.png"),
    (400, 300, "After Convolution", "after_convolution.png"),
    (1400, 1000, "Google Sheets Full View", "sheets_full_view.png"),
    (800, 600, "Sheets Zoomed View", "sheets_zoomed.png"),
    (600, 400, "Cell Properties", "cell_properties.png"),
]

print("Generating placeholder images...")
for width, height, text, filename in placeholders:
    create_placeholder(width, height, text, filename)

print("\nAll placeholders created successfully!")
print("Place them in the 'images/' directory")
```

Run the script:

```bash
pip install pillow
python generate_placeholders.py
```

## Method 3: PowerPoint/Draw.io

1. **Create diagrams** in PowerPoint or Draw.io:
   - System Block Diagram (block_diagram.png)
   - OAuth Flow Diagram (oauth_flow.png)

2. **Export as PNG** with high resolution (1200x800 or larger)

3. **Save to images/** directory

## Method 4: Actual Screenshots

### For CLI Screenshots:

1. **Run your application** in the terminal
2. **Take screenshots** at key points:
   - Startup prompt (cli_startup.png)
   - OAuth URL display (oauth_url.png)
   - Success message (auth_complete.png)

3. **Tools:**
   - Windows: Win + Shift + S (Snipping Tool)
   - Mac: Cmd + Shift + 4
   - Linux: gnome-screenshot or Flameshot

### For Google Sheets Screenshots:

1. **Open your rendered spreadsheet**
2. **Take full view screenshot** (sheets_full_view.png)
3. **Zoom in** on cells and take close-up (sheets_zoomed.png)
4. **Right-click a cell** → Format → Take screenshot (cell_properties.png)

### For OAuth Screenshots:

1. **Run OAuth flow**
2. **Screenshot Google consent page** (google_consent.png)
3. Make sure to **blur/hide** any sensitive information

## Image Directory Structure

Create this structure:

```
report/
├── main.tex
├── chapters/
│   └── ...
└── images/              <- Create this directory
    ├── logo.png
    ├── block_diagram.png
    ├── oauth_flow.png
    ├── cli_startup.png
    ├── oauth_url.png
    ├── google_consent.png
    ├── auth_complete.png
    ├── input_sample.jpg
    ├── resized_image.png
    ├── before_convolution.png
    ├── after_convolution.png
    ├── sheets_full_view.png
    ├── sheets_zoomed.png
    └── cell_properties.png
```

## Creating Diagrams

### System Block Diagram

Should show:

```
┌─────────────┐
│ Input Image │
└──────┬──────┘
       │
       ▼
┌──────────────────┐
│ Resize (400x300) │
└────────┬─────────┘
         │
         ▼
┌─────────────────┐
│  Convolution    │
│   (Sharpen)     │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│   Format Data   │
│  (RGB → JSON)   │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Google Sheets  │
│   API Upload    │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│    Rendered     │
│   Spreadsheet   │
└─────────────────┘
```

### OAuth Flow Diagram

Should show:

```
User          App           Browser       Google
 │             │                │            │
 │   Start     │                │            │
 ├────────────►│                │            │
 │             │  User URL      │            │
 │             ├───────────────►│            │
 │             │                │   Auth     │
 │             │                ├───────────►│
 │             │                │  Consent   │
 │             │                │◄───────────┤
 │             │   Callback     │            │
 │             │◄───────────────┤            │
 │             │  Get Token     │            │
 │             ├───────────────────────────►│
 │             │   Access Token │            │
 │             │◄───────────────────────────┤
 │   Success   │                │            │
 │◄────────────┤                │            │
```

## Tips for Better Images

1. **High Resolution:** Use at least 1200x800 for screenshots
2. **Clean Background:** Use plain terminals without clutter
3. **Readable Text:** Ensure font sizes are legible
4. **Consistent Style:** Use similar color schemes
5. **Annotations:** Add arrows or labels if needed

## Compiling Without Images

If you want to compile the document before adding images:

1. **Comment out figure includes** in chapter files:

```latex
% \includegraphics[width=0.9\textwidth]{oauth_flow.png}
```

2. **Or use draft mode** in main.tex:

```latex
\usepackage[draft]{graphicx}  % Shows placeholder boxes instead
```

This will compile successfully with placeholder boxes instead of images.

## Next Steps

1. Create the `images/` directory
2. Generate or download placeholder images
3. Test compilation with: `compile.bat` (Windows) or `./compile.sh` (Linux/Mac)
4. Replace placeholders with actual screenshots as you create them
5. Recompile to see updates

---

**Note:** The LaTeX document will compile with warnings if images are missing, but will still generate a PDF with "[Image not found]" placeholders.
