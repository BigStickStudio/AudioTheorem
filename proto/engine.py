# 2024-05-20 R. Chris Ancillary

import pygame
import math


class Engine: 
    def __init__(self):
        # Set up the window
        self.SCREEN_WIDTH = 1600
        self.SCREEN_HEIGHT = 1200
        self.clock = pygame.time.Clock()

        # Mechanics
        self.running = True
        self.waveforms = []

        #Pregame
        self.Frequency = 432
        self.Amplitude = 100
        self.Phase = 0
        self.Time = 0
        self.SAMPLES = 64    # Ideally would be 2048 or higher - Proprietary Ancillary, Tonal Synthesis Superposition
        self.SAMPLE_RATE = 44100

        # Initialize Pygame
        pygame.init()
        self.window = pygame.display.set_mode((self.SCREEN_WIDTH, self.SCREEN_HEIGHT))
        pygame.display.set_caption("Waveform Visualizer")

    def handle_events(self):
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                self.running = False


    # Phase radius is representative of the amplitude of the wave
    # Our Frequency Band spans from 8.025hz and goes to 32.039khz
    # An Octave is a subdivision of 12 semitone intervals where a given tone can be subtone into 5 subtones representing 200 cents each

    #  Mipmapping Methodology
    #  100 cents = .5 subtone
    #  200 cents = 1 subtone
    #  5 subtones = 1 semitone
    #  12 semitones = 1 octave
    #  1200 cents = 1 octave
    #  
    #def step(self):

    # Filters are wavetables - Let's go tensors!
    def update(self):
        return

    def draw(self):
        self.window.fill((0, 0, 0))
        pygame.draw.line(self.window, (255, 255, 255), (0, self.SCREEN_HEIGHT // 2), (self.SCREEN_WIDTH, self.SCREEN_HEIGHT // 2))
        pygame.draw.line(self.window, (255, 255, 255), (self.SCREEN_WIDTH // 2, 0), (self.SCREEN_WIDTH // 2, self.SCREEN_HEIGHT))

        for x in range(self.SCREEN_WIDTH):
            y = self.Amplitude * math.sin((2 * math.pi * self.Frequency * self.Time) + self.Phase)
            y = int(y)

            pygame.draw.circle(self.window, (255, 255, 255), (x, self.SCREEN_HEIGHT // 2 - y), 1)

            self.Time += 1 / self.SAMPLES

        pygame.display.update()
        pygame.time.delay(100)

        self.Time = 0
        self.Phase += 0.1

    def run(self):
        while self.running:
            self.handle_events()
            self.update()
            self.draw()
            self.clock.tick(60)

