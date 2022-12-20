from math import sin, cos
from PIL import Image
import sqlite3


def cartesian(r, phi):
	return (round(r * cos(phi)), round(r * sin(phi)))


def main():
	with sqlite3.connect('challenge.db') as conn:
		c = conn.cursor()
		w, h = c.execute('SELECT width, height FROM image;').fetchone()
		im = Image.new('1', (w, h))
		c.execute('SELECT r, phi, colour FROM pixels;')
		for r, phi, colour in c.fetchall():
			x, y = cartesian(r, phi)
			im.putpixel((x, y), colour)

	im.save('image.png')


if __name__ == '__main__':
	main()