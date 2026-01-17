"""
Placeholder Image Generator for Sheesee Report
Generates all required placeholder images for the LaTeX report
"""

try:
    from PIL import Image, ImageDraw, ImageFont
except ImportError:
    print("ERROR: PIL (Pillow) not found.")
    print("Install with: pip install pillow")
    exit(1)

import os

def create_placeholder(width, height, text, filename, bg_color='#E8E8E8', text_color='#666666'):
    """Create a placeholder image with centered text"""
    # Create image
    img = Image.new('RGB', (width, height), color=bg_color)
    draw = ImageDraw.Draw(img)
    
    # Try to load a font, fall back to default
    font_size = min(width, height) // 15
    try:
        # Try common font locations
        font_options = [
            "arial.ttf",
            "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
            "/System/Library/Fonts/Helvetica.ttc",
            "C:\\Windows\\Fonts\\arial.ttf"
        ]
        font = None
        for font_path in font_options:
            try:
                font = ImageFont.truetype(font_path, font_size)
                break
            except:
                continue
        
        if font is None:
            font = ImageFont.load_default()
    except:
        font = ImageFont.load_default()
    
    # Calculate text bounding box and position
    bbox = draw.textbbox((0, 0), text, font=font)
    text_width = bbox[2] - bbox[0]
    text_height = bbox[3] - bbox[1]
    
    position = ((width - text_width) // 2, (height - text_height) // 2)
    
    # Draw border
    border_width = 3
    draw.rectangle(
        [border_width, border_width, width - border_width, height - border_width],
        outline='#CCCCCC',
        width=border_width
    )
    
    # Draw text
    draw.text(position, text, fill=text_color, font=font)
    
    # Draw dimensions text at bottom
    dims_text = f"{width} × {height}"
    dims_bbox = draw.textbbox((0, 0), dims_text, font=font)
    dims_width = dims_bbox[2] - dims_bbox[0]
    dims_position = ((width - dims_width) // 2, height - 40)
    draw.text(dims_position, dims_text, fill='#999999', font=font)
    
    # Save image
    img.save(filename)
    return True

def main():
    # Create images directory if it doesn't exist
    images_dir = "images"
    if not os.path.exists(images_dir):
        os.makedirs(images_dir)
        print(f"Created directory: {images_dir}/")
    
    # Define all placeholders
    placeholders = [
        # (width, height, text, filename)
        (300, 300, "University Logo", "logo.png"),
        (1200, 800, "System Block Diagram", "block_diagram.png"),
        (1000, 600, "OAuth Flow Diagram", "oauth_flow.png"),
        (1200, 800, "CLI Startup Screen", "cli_startup.png"),
        (1200, 800, "OAuth URL Display", "oauth_url.png"),
        (1200, 800, "Google Consent Page", "google_consent.png"),
        (1200, 800, "Authentication Complete", "auth_complete.png"),
        (800, 600, "Sample Input Image", "input_sample.jpg"),
        (400, 300, "Resized 400×300", "resized_image.png"),
        (400, 300, "Before Convolution", "before_convolution.png"),
        (400, 300, "After Convolution", "after_convolution.png"),
        (1400, 1000, "Google Sheets\nFull View", "sheets_full_view.png"),
        (800, 600, "Sheets Zoomed View", "sheets_zoomed.png"),
        (600, 400, "Cell Properties", "cell_properties.png"),
    ]
    
    print("=" * 50)
    print("  Sheesee Report - Placeholder Image Generator")
    print("=" * 50)
    print()
    print(f"Generating {len(placeholders)} placeholder images...")
    print()
    
    success_count = 0
    for width, height, text, filename in placeholders:
        filepath = os.path.join(images_dir, filename)
        try:
            create_placeholder(width, height, text, filepath)
            print(f"✓ Created: {filepath}")
            success_count += 1
        except Exception as e:
            print(f"✗ Failed: {filepath} - {e}")
    
    print()
    print("=" * 50)
    print(f"Complete! {success_count}/{len(placeholders)} images created")
    print("=" * 50)
    print()
    print("Next steps:")
    print("1. Replace placeholders with actual screenshots")
    print("2. Run compile.bat (Windows) or ./compile.sh (Linux/Mac)")
    print("3. Check main.pdf for results")
    print()

if __name__ == "__main__":
    main()
