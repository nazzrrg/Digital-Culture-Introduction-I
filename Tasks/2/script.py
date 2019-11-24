from skimage.io import imread  # подключение библиотеки ввыода картинки
import collections  # подключение библиотеки для определения частоты символов алфавита
import math  # подключение библиотеки с логарифмами

line = imread("image.jpg", as_gray=True)[64]  # чтение картинки в grayscale и выдергивание из нее средней строки

for i in range(128):
    line[i] = round(int(round(line[i] * 255)) / 20) * 20  # квантование строки

print(str(line).replace(".", ","))  # вывод отквантованной строки

n_items = list(collections.Counter(line).items())  # подсчет числа различных значений

print(n_items)  # вывод алфаваита с количеством появлений

entropy = 0  # Вычисление энтропии

for i in range(8):
    entropy += (n_items[i][1] / 128) * math.log2(n_items[i][1] / 128)

entropy *= -1

print("Entropy: ", round(entropy, 6))  # Вывод энтропии

line_ShannonFano = str(line)
line_Haffman = str(line)
line_UniformBinaryCode = str(line)

print(line_ShannonFano.replace(",", "").replace(" ", "").replace("120", "11").replace("140", "10").replace("100", "011")
      .replace("80", "010").replace("160", "0001").replace("60", "00001").replace("40", "000001")
      .replace("20", "000000").replace(".", "").replace("\n", ""))  # замена символов на код Шеннона-Фано

print(line_Haffman.replace(",", "").replace(" ", "").replace("120", "1").replace("140", "00").replace("100", "011")
      .replace("80", "0101").replace("160", "01001").replace("60", "010000").replace("40", "0100010")
      .replace("20", "010001").replace(".", "").replace("\n", ""))  # замена символов на код Хаффмана

print(line_UniformBinaryCode.replace(",", "").replace(" ", "").replace("120", "101").replace("140", "110")
      .replace("100", "100").replace("80", "011").replace("160", "111").replace("60", "010")
      .replace("40", "001").replace("20", "000").replace(".", "").replace("\n", ""))
      # замена символов на равномерный двоичный код
