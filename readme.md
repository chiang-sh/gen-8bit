This is a small tool for converting images to an 8-bit pixel art style.

See the documentation for a usage example.

# Processing Flow
1. Read an image from a local path.
2. Downscale the image to reduce detailed color information.
3. Generate a 16-color palette using the Median Cut algorithm. (The palette size may become configurable in a future release)
4. Quantize the image colors using the palette.
5. Save the processed image to the specified path.

# Results
<img width="761" height="253" alt="result1" src="https://github.com/user-attachments/assets/f982c5d9-456d-4d6d-a258-c71f9bd25ee3" />
<img width="761" height="564" alt="result2" src="https://github.com/user-attachments/assets/3ed77721-5846-458d-88f2-ac4d5ba9ee10" />

# References
* https://web.ntnu.edu.tw/~algo/Image.html
* https://hackmd.io/@chrish0729/SkpfuvZx6
* https://ithelp.ithome.com.tw/m/articles/10404368
* https://en.wikipedia.org/wiki/Color_quantization
* https://en.wikipedia.org/wiki/Median_cut
