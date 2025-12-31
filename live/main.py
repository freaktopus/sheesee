import cv2
import numpy as np
import sys

# Grid size
ROWS = 30
COLS = 80

# ASCII characters by density
ASCII_CHARS = "@%#*+=-:. "

def capture_frame():
    cap = cv2.VideoCapture(0)
    ret, frame = cap.read()
    cap.release()
    if not ret:
        return None
    # Resize to terminal grid
    small = cv2.resize(frame, (COLS, ROWS), interpolation=cv2.INTER_AREA)
    return small

def frame_to_ascii(frame):
    """Convert frame to ASCII chars with ANSI color."""
    ascii_frame = []
    for row in frame:
        line = ""
        for pixel in row:
            b, g, r = pixel
            # Convert pixel brightness for ASCII selection
            brightness = int(0.299*r + 0.587*g + 0.114*b)
            char = ASCII_CHARS[brightness * (len(ASCII_CHARS)-1)//255]
            # Add ANSI color (foreground)
            line += f"\033[38;2;{r};{g};{b}m{char}\033[0m"
        ascii_frame.append(line)
    return ascii_frame

def draw_terminal(ascii_frame):
    print("\033[H", end="")  # Move cursor to top-left
    for line in ascii_frame:
        print(line)
    sys.stdout.flush()

if __name__ == "__main__":
    # Hide cursor
    print("\033[?25l", end="")
    try:
        while True:
            frame = capture_frame()
            if frame is not None:
                ascii_frame = frame_to_ascii(frame)
                draw_terminal(ascii_frame)
    except KeyboardInterrupt:
        # Show cursor on exit
        print("\033[?25h")
        sys.exit()

