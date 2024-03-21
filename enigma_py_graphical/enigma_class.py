from components import *

class Enigma:
    def __init__(self, plug_args=[], rotoriii_setup=2, rotorii_setup=1, rotori_setup=0, reflector_setup='A'):
        self.keyboard  = Keyboard()
        self.plugboard = Plugboard(plug_args)
        self.rotoriii  = Rotor(rotoriii_setup)
        self.rotorii   = Rotor(rotorii_setup)
        self.rotori    = Rotor(rotori_setup)
        self.reflector = Reflector(reflector_setup)

    def set_rotor_key(self, key):
        self.rotoriii.rotate_to_char(key[0])
        self.rotorii.rotate_to_char(key[1])
        self.rotori.rotate_to_char(key[2])

    def encipher(self, letter):

        # Rotasjon brorsan
        if self.rotorii.left[0] == self.rotorii.turnover_notch and self.rotoriii.left[0] == self.rotoriii.turnover_notch:
            self.rotori.rotate()
            self.rotorii.rotate()
            self.rotoriii.rotate()

        elif self.rotoriii.left[0] == self.rotoriii.turnover_notch:
            self.rotorii.rotate()
            self.rotoriii.rotate()

        else:
            self.rotoriii.rotate()

        sig = self.keyboard.forward(letter)
        print(sig)
        sig = self.plugboard.forward(sig)
        print(sig)
        sig = self.rotoriii.forward(sig)
        print(sig)
        sig = self.rotorii.forward(sig)
        print(sig)
        sig = self.rotori.forward(sig)
        print(sig)
        sig = self.reflector.reflect(sig)
        print(sig)
        sig = self.rotori.backward(sig)
        print(sig)
        sig = self.rotorii.backward(sig)
        print(sig)
        sig = self.rotoriii.backward(sig)
        print(sig)
        sig = self.plugboard.backward(sig)
        print(sig)
        letter = self.keyboard.backward(sig)
        return letter

#en = Enigma()
#en.set_rotor_key('AAA')
#en.rotoriii.show()
#en.rotorii.show()
#en.rotori.show()

#while True:
#    print(en.encipher(input(': ').upper()))
    #en.rotorii.show()
