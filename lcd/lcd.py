from time import sleep
from Adafruit_CharLCD import Adafruit_CharLCD

lcd = Adafruit_CharLCD(rs=22, en=5, d4=26,d5=19,d6=13,d7=6,cols=16,lines=2)

lcd.clear()

lcd.message('Hello\n world!')
sleep(3)

sleep(3)