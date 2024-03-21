import pygame as pg 
from enigma_class import Enigma 

# pygame initialization
pg.init()
pg.font.init()
pg.display.set_caption("Arne's Enigma Simulator")

# Font creation
MONO = pg.font.SysFont('Amiri', 25)
BOLD = pg.font.SysFont('Amiri', 25, bold=True)

# clock initialization
clock = pg.time.Clock()

# global variables
SWIDTH = 1600
SHEIGHT = 900
SCREEN = pg.display.set_mode((SWIDTH, SHEIGHT))

en = Enigma(plug_args=['AB'], rotori_setup=1, rotorii_setup=1, rotoriii_setup=1, reflector_setup='A')

print(en.plugboard.left)

# gameloop
running = True
while running:
    # Limit FPS
    clock.tick(60)

    # background colour
    SCREEN.fill('#333333')

    # draw keyboard
    en.keyboard.draw(SCREEN, (1400,100), (50,700), BOLD)
    
    # draw plugboard
    en.plugboard.draw(SCREEN, (1150,100), (100, 700), BOLD)

    # draw rotors
    en.rotoriii.draw(SCREEN, (750+150, 100), (100,700), BOLD)
    en.rotorii.draw(SCREEN, (750, 100), (100,700), BOLD)
    en.rotori.draw(SCREEN, (750-150, 100), (100,700), BOLD)

    # draw reflector
    en.reflector.draw(SCREEN, (150, 100), (100,700), BOLD)

    # update the screen
    pg.display.flip()

    for event in pg.event.get():
        if event.type == pg.QUIT:
            running = False

        if event.type == pg.KEYDOWN:

            #en.rotoriii.rotate()
            try:
                letter_pressed = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'[event.key - 97]
                letter_encrypted = en.encipher(letter_pressed)

                print(f'{letter_pressed} : {letter_encrypted}')
            except IndexError:
                print("Try another key.")


