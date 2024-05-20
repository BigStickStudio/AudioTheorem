# 2024-05-20 R. Chris Ancillary

import pygame
import math
from frame import Frame
from resolution import Resolution

class Engine: 
    def __init__(self):
        # Set up the window
        self.SCREEN_WIDTH = 1800
        self.SCREEN_HEIGHT = 1200
        self.HALF_WIDTH = self.SCREEN_WIDTH // 2
        self.HALF_HEIGHT = self.SCREEN_HEIGHT // 2

        # State
        self.running = True
        self.waveforms = []

        # Configuration
        self.min_freq = 8.025
        self.max_freq = 32039
        self.resolution = Resolution.SEMITONE

        self.Frequency = 880
        self.Amplitude = 100
        self.Phase = 0
        self.Time = 0
        self.SAMPLES = 2048              # Ideally would be 2048 or higher - Proprietary Ancillary, Tonal Synthesis Superposition
        self.SAMPLE_RATE = 44100        # How fast we sample the wave

        # Initialize Pygame
        self.window = pygame.display.set_mode((self.SCREEN_WIDTH, self.SCREEN_HEIGHT))

        # Initialize Frames
        self.top_shelf = Frame(self.window, 0, 0, self.SCREEN_WIDTH, self.HALF_HEIGHT)
        self.bottom_left = Frame(self.window, 0, self.HALF_HEIGHT, self.HALF_WIDTH, self.HALF_HEIGHT)
        self.bottom_right = Frame(self.window, self.HALF_WIDTH, self.HALF_HEIGHT, self.HALF_WIDTH, self.HALF_HEIGHT)

        # Initialize Engine
        self.clock = pygame.time.Clock()
        pygame.init()
        pygame.display.set_caption("Waveform Visualizer")


    def handle_events(self):
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                self.running = False
            
            if event.type == pygame.KEYDOWN:
                if event.key == pygame.K_ESCAPE:
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
        self.Time += 1 / self.SAMPLES
        # Need to calculate theta for our phase in terms of X, Y = (Phase, Amplitude) where our Amplitude is our radius and our Phase is the angle in radians
        #self.Phase = 2 * math.pi * self.Frequency * self.Time

        return

    # TOP - Waveform Shape in Y and Z axis (Amplitude @ Frequency)
    def draw_waveform(self): # This is how class inheritance is created
        self.top_shelf.draw_midline()
        self.top_shelf.draw_frame()

        for idx in range(self.SAMPLES):
            # Calculate the Y value for the wave
            y = self.Amplitude * math.sin(self.Phase) 

            # TODO: Implement Unit Circle Mix Radius for a more dynamic visualization
            # Draw the point
            (relative_x, relative_y) = self.top_shelf.relative_offset(idx, self.SAMPLES, y, self.Amplitude)
            pygame.draw.circle(self.window, (255, 255, 255), (relative_x, relative_y), 1)

            # Increment the phase
            self.Phase += 2 * math.pi * self.Frequency / self.SAMPLE_RATE

        return

    # BOTTOM LEFT - { X axis: Phase,  Y axis: Amplitude }
    def draw_circles(self):
        self.bottom_left.draw_cross()
        self.bottom_left.draw_frame()

        # For each wave, draw a circle at the corresponding phase and amplitude
        phase_x = self.Phase
        amplitude_y = self.Amplitude

        # Calculate the X and Y values for the wave
        x = amplitude_y * math.cos(phase_x)
        y = amplitude_y * math.sin(phase_x)

        (relative_x, relative_y) = self.bottom_left.radial_offset(x, self.Amplitude, y, self.Amplitude)
        pygame.draw.circle(self.window, (255, 255, 255), (relative_x, relative_y), 1)
        

        return

    # TODO: Make this 3D and merge the 3 into a dynamic 4D visualization
    # BOTTOM RIGHT - { Subtones per Octave - 0 - 144 ~|| 8hz - 32khz across 60 subtones per octave == 720 subtones || 1440 cents }
    def draw_bands(self):
        self.bottom_right.draw_verticles()
        self.bottom_right.draw_frame()

        # For each band, draw a line at the corresponding amplitude which is frame_height / total amplitude
        return


    def draw(self):
        self.window.fill((0, 0, 0))

        self.draw_waveform()
        self.draw_circles()
        self.draw_bands()

        pygame.display.update()

    def run(self):
        while self.running:
            self.handle_events()
            self.update()
            self.draw()
            self.clock.tick(60) # This needs to be configured in a way where we are not blocking or multi-processing the same data .. we need a proper update loop

