# // Position = 0.0     Color = (  0,   7, 100)
# // Position = 0.16    Color = ( 32, 107, 203)
# // Position = 0.42    Color = (237, 255, 255)
# // Position = 0.6425  Color = (255, 170,   0)
# // Position = 0.8575  Color = (  0,   2,   0)

from scipy.interpolate import PchipInterpolator as interpolate

x = [0., .16, .42, .6425, .8575, 1.]
yr = [0, 32, 237, 255, 0, 0]
yg = [7, 107, 255, 170, 2, 7]
yb = [100, 203, 255, 0, 0, 100]

ir = interpolate(x, yr)
ig = interpolate(x, yg)
ib = interpolate(x, yb)

colours = set()
with open("palette.txt", "w") as f:
    f.write("const PALETTE: [u32; 1024] = [")
    for row in range(128):
        for col in range(8):
            j = col + 8 * row
            colour = int(ir(j/1024)) + 2**8 * int(ig(j/1024)) + 2**16 * int(ib(j/1024)) + 255 * 2**24
            colours.add(colour)
            f.write(f"0x{colour:x}, ")
        f.write("\n")
    f.write("];")

print(len(colours))