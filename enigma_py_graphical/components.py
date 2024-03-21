import pygame

class Keyboard:
    def __init__(self):
        self.ab = list('ABCDEFGHIJKLMNOPQRSTUVWXYZ')

        self.highlighted_letter = None # placeholder
        
        self.output_letter = None

    def forward(self, c):
        self.highlighted_letter = self.ab.index(c.upper())

        return self.ab.index(c.upper())

    def backward(self, c_index):
        self.output_letter = c_index

        return self.ab[c_index]

    def draw(self, screen, pos, size, font):
        x, y = pos 
        width, height = size

        # Draw rectangle
        r = pygame.Rect(x,y,width,height)
        pygame.draw.rect(screen, 'yellow', r, width=2, border_radius=15)

        # Text 
        for i in range(26):
            letter = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'[i]
            letter = font.render(letter, True, 'grey')
            
            text_box = letter.get_rect(center = (width/2 + x, y + (i+1) * height/27))

            if self.highlighted_letter != None:
                if self.ab[i] == self.ab[self.highlighted_letter]:
                    pygame.draw.rect(screen, 'green', text_box, border_radius = 35)
                    letter = self.ab[self.highlighted_letter]
                    letter = font.render(letter, True, 'red')
            
            if self.output_letter != None:
                if self.ab[i] == self.ab[self.output_letter]:
                    pygame.draw.rect(screen, 'yellow', text_box, border_radius = 35)
                    letter = self.ab[self.output_letter]
                    letter = font.render(letter, True, 'blue')

            screen.blit(letter, text_box)

class Plugboard:
    def __init__(self, pair_plugs):
        # Define all variables used
        self.left  = list('ABCDEFGHIJKLMNOPQRSTUVWXYZ')
        self.right = list('ABCDEFGHIJKLMNOPQRSTUVWXYZ')
        self.char_i = ''
        self.char_ii = ''
        self.char_i_pos = 0
        self.char_ii_pos = 0

        # For loop to switch the positions of letter pairs.
        for pair_plug in pair_plugs:
            self.char_i = pair_plug[0]
            self.char_ii = pair_plug[1]

            self.char_i_pos = self.left.index(self.char_i)
            self.char_ii_pos = self.left.index(self.char_ii)

            self.left.remove(self.char_i)
            self.left.remove(self.char_ii)

            self.left.insert(self.char_i_pos, self.char_ii)
            self.left.insert(self.char_ii_pos, self.char_i)
    
    def forward(self, c_index):
        """
        Finds out what index the specified character is on the next 'row' of letters.
        """
        right_char = self.right[c_index]
        left_index = self.left.index(right_char)
        return left_index

    def backward(self, c_index):
        """
        The inverse of forward func.
        """
        left_char = self.left[c_index]
        right_index = self.right.index(left_char)
        return right_index

    def draw(self, screen, pos, size, font):
        x, y = pos
        width, height = size 

        # Draw rectangle 
        r = pygame.Rect(x,y,width,height)
        pygame.draw.rect(screen, 'red', r, width=2, border_radius=15)

        # Textorama
        # Right
        for i in range(26):
            letter = self.right[i]
            letter = font.render(letter, True, 'grey')

            text_box = letter.get_rect(center=(width/4 * 3 + x, y + (i+1) * height/27))
            screen.blit(letter, text_box)

        # Text Left 
        for i in range(26):
            letter = self.left[i]
            letter = font.render(letter, True, 'grey')

            text_box = letter.get_rect(center=(width/4 * 1 + x, y + (i+1)*height/27))
            screen.blit(letter, text_box)
            


class Rotor:
    # For information on rotor details, look up wikipedia.
    def __init__(self, wiring_template):
        self.left = list('ABCDEFGHIJKLMNOPQRSTUVWXYZ')
        self.right = None

        if wiring_template == 0:
            self.right = list('EKMFLGDQVZNTOWYHXUSPAIBRCJ')
            self.turnover_notch = 'Q'

        elif wiring_template == 1:
            self.right = list('AJDKSIRUXBLHWTMCQGZNPYFVOE')
            self.turnover_notch = 'E'

        elif wiring_template == 2:
            self.right = list('BDFHJLCPRTXVZNYEIWGAKMUSQO')
            self.turnover_notch = 'V'

        elif wiring_template == 3:
            self.right = list('ESOVPZJAYQUIRHXLNFTGKDCMWB')
            self.turnover_notch = 'J'

        elif wiring_template == 4:
            self.right = list('VZBRGITYUPSDNHLXAWMJQOFECK')
            self.turnover_notch = 'Z'
            
            
        print(f"config: {wiring_template} - {self.right}")

    def forward(self, c_index):
        right_char = self.right[c_index]
        return self.left.index(right_char)

    def backward(self, c_index):
        left_char = self.left[c_index]
        return self.right.index(left_char)

    def rotate(self):
        moved_char = self.left[0]
        self.left.pop(0)
        self.left.append(moved_char)

        moved_char = self.right[0]
        self.right.pop(0)
        self.right.append(moved_char)

    def rotate_to_char(self, _char):
        _char = _char.upper()

        while True:
            if self.left[0] != _char:
                self.rotate()
            elif self.left[0] == _char:
                return

    def show(self):
        print(f"LS {self.left}")
        print(f"RS {self.right}")
        print(" ")

    def draw(self, screen, pos, size, font):
        x, y = pos 
        width, height = size 

        # Draw rect 
        r = pygame.Rect(x,y,width,height)
        pygame.draw.rect(screen, 'white', r, width = 2, border_radius=15)


        # Textorama Right 
        for i in range(26):
            letter = self.right[i]
            letter = font.render(letter, True, 'grey')
            text_box = letter.get_rect(center = (width / 4 * 3 + x, y + (i + 1) * height / 27 ))

            if self.left[i] == self.turnover_notch:
                pygame.draw.rect(screen, 'white', text_box, border_radius = 35)
                letter = self.right[i]
                letter = font.render(letter, True, 'black')

            screen.blit(letter, text_box)
        # Textorama Left 
        for i in range(26):
            letter = self.left[i]
            letter = font.render(letter, True, 'grey')
            text_box = letter.get_rect(center = (width / 4 * 1 + x, y + (i + 1) * height / 27 ))
            screen.blit(letter, text_box)


class Reflector:
    def __init__(self, reflector_template):
        self.left = list('ABCDEFGHIJKLMNOPQRSTUVWXYZ')
        
        if reflector_template == 'A':
            self.right = list('EJMZALYXVBWFCRQUONTSPIKHGD')
        elif reflector_template == 'B':
            self.right = list('YRUHQSLDPXNGOKMIEBFZCWVJAT')
        elif reflector_template == 'C':
            self.right = list('FVPJIAOYEDRZXWGCTKUQSBNMHL')

    def reflect(self, c_index):
        right_char = self.right[c_index]
        return self.left.index(right_char)

    def draw(self, screen, pos, size, font):
        x,y = pos
        width, height = size 

        # Draw rectangle
        r = pygame.Rect(x,y,width,height)
        pygame.draw.rect(screen, 'yellow', r, width=2, border_radius=15)

        # Textorama Hoyre
        for i in range(26):
            letter = self.right[i]
            letter = font.render(letter, True, 'grey')

            text_box = letter.get_rect(center = (width / 4 * 3 + x, y + (i+1) * height / 27))
            screen.blit(letter, text_box)


        for i in range(26):
            letter = self.left[i]
            letter = font.render(letter, True, 'grey')

            text_box = letter.get_rect(center = (width / 4 * 1 + x, y + (i+1) * height / 27))
            screen.blit(letter, text_box)

