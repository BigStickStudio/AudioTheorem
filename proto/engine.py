# 2024-05-20 R. Chris Ancillary

import pygame
import math
import time
from frame import Frame
from resolution import Resolution
from colors import Color
from waveform import WaveForm, Wave

class Engine: 
    def __init__(self):
        # Set up the window
        self.SCREEN_WIDTH = 1200
        self.SCREEN_HEIGHT = 1200
        self.HALF_WIDTH = self.SCREEN_WIDTH // 2
        self.HALF_HEIGHT = self.SCREEN_HEIGHT // 2

        self.alpha_xyz = [0, 0, 0] # Alpha represents the amplitude, phase, and sample index

        # State
        self.SAMPLE_RATE = 44100
        self.SAMPLES = 2048
        self.running = True
        self.waveforms = []
        self.max_amplitude = 150 # TODO: Make this more dynamic
        self.waveforms.append(WaveForm("Sine", len(self.waveforms), 220.0, 20, self.SAMPLES, self.SAMPLE_RATE))
        self.waveforms.append(WaveForm("Sine", len(self.waveforms), 440.0, 40, self.SAMPLES, self.SAMPLE_RATE))
        self.waveforms.append(WaveForm("Sine", len(self.waveforms), 880.0, 60, self.SAMPLES, self.SAMPLE_RATE))
        
        self.superposition = WaveForm("Superposition", 999, 0.0, 0)
        self.super_index = 0

        for wave in self.waveforms:
            self.superposition.add(wave)
        

        # Configuration
        self.min_freq = 8.025
        self.max_freq = 32039
        self.frequency_range = self.max_freq - self.min_freq
        self.resolution = Resolution.SEMITONE
        self.frame_spacing = 3

        # Initialize Pygame
        self.window = pygame.display.set_mode((self.SCREEN_WIDTH, self.SCREEN_HEIGHT))

        # Initialize Frames
        self.top_shelf = Frame(self.window, 0, 0, self.SCREEN_WIDTH, self.HALF_HEIGHT, self.frame_spacing)
        self.bottom_left = Frame(self.window, 0, self.HALF_HEIGHT, self.HALF_WIDTH, self.HALF_HEIGHT, self.frame_spacing)
        self.bottom_right = Frame(self.window, self.HALF_WIDTH, self.HALF_HEIGHT, self.HALF_WIDTH, self.HALF_HEIGHT, self.frame_spacing)

        # Initialize Engine
        self.clock = pygame.time.Clock()
        pygame.init()
        pygame.display.set_caption("Waveform Visualizer")

        # Initialize Clock Cycle
        self.last_time = time.time()
        self.now = time.time()
        self.delta_time = self.now - self.last_time
        self.wait_time = 0
        self.last_time = self.now

    def cyclic_time(self):
        # returns true if the delta time is greater than the sample rate
        self.wait_time += self.delta_time
        return self.wait_time > 1 / self.SAMPLE_RATE


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
        # We need to figure out what this looks like based on the parent refresh rate - and it really should be bound to `now` - `last_time`
        self.now = time.time()
        self.delta_time = self.now - self.last_time
        self.last_time = self.now

        # If we haven't reached the sample rate, we don't want to update the waveforms
        if not self.cyclic_time():
            return False

        # TODO: Update the waveforms here?
        #for wave in self.waveforms:
        #    wave.next_sample() 

        # copy any waveforms to superposition
        #self.waveforms.clear()

        #for wave in self.waveforms:
        #    self.superposition.add(wave)

        return True

    def draw_shapes(self):
        # Draw the top shelf
        self.top_shelf.draw_midline()
        self.top_shelf.draw_frame()

        # Draw the bottom left shelf
        self.bottom_left.draw_cross()
        self.bottom_left.draw_frame()

        # Draw the bottom right shelf
        self.bottom_right.draw_verticles()
        self.bottom_right.draw_frame()

    # TODO: Make this 3D and merge the 3 into a dynamic 4D visualization
    # BOTTOM RIGHT - { Subtones per Octave - 0 - 144 ~|| 8hz - 32khz across 60 subtones per octave == 720 subtones || 1440 cents }
    def draw_bands(self):
        n_bands = self.resolution.subdivisions()
        bandwidth_range = self.frequency_range / n_bands
        bandwidth_width = self.bottom_right.width / n_bands
        # For each band, draw a line at the corresponding amplitude which is frame_height / total amplitude

        for n in range(n_bands):
            #TODO: We need to calculate undertones and overtones for a given frequency
            band_freq_start = self.min_freq + n * bandwidth_range
            band_freq_close = self.min_freq + (n + 1) * bandwidth_range

            ## WE ARE ITERATING AND COMPOUNDING ALL OF THE WAVEFORMS HERE ##
            # If the band is one of the wavelengths +/- the start and close of the band, we want to draw a line
            some_wave = next((wave for wave in self.waveforms if band_freq_start < wave.frequency <= band_freq_close), None)

            octave_color = Color.get(n // 12)
            pitchclass_color = Color.get(n % 12)
            final_color = octave_color.lerp(pitchclass_color, 0.56)
            amplitude = 100
            t_range = 100

            if some_wave is not None:
                (left_x, top_y) = self.bottom_right.bandwidth_offset(n, n_bands, some_wave.amplitude, t_range)
                rect = pygame.Rect(left_x, top_y, bandwidth_width, some_wave.amplitude * 4)
                continue

            (left_x, top_y) = self.bottom_right.bandwidth_offset(n, n_bands, 10, t_range) 
            rect = pygame.Rect(left_x, top_y, bandwidth_width, 20)
            pygame.draw.rect(self.window, final_color, rect)
            

    def draw_positions(self):
        
        n_bands = self.resolution.subdivisions()

        for sample in self.superposition.samples:
            (top_shelf_x, top_shelf_y) = self.top_shelf.wavelength_offset(self.super_index, len(self.superposition.samples), sample.y, self.max_amplitude)
            pygame.draw.circle(self.window, Color.GRAY.value, (top_shelf_x + self.frame_spacing, top_shelf_y), 1)

            (bottom_left_x, bottom_left_y) = self.bottom_left.radial_offset(sample.x, self.max_amplitude, sample.y, self.max_amplitude)
            pygame.draw.circle(self.window, Color.GRAY.value, (bottom_left_x + self.frame_spacing, bottom_left_y), 1)


        for wave in self.waveforms:
            for sample in wave.samples:
                sample_index = wave.samples.index(sample)
                (top_shelf_x, top_shelf_y) = self.top_shelf.wavelength_offset(sample_index, len(wave.samples), sample.y, self.max_amplitude)
                pygame.draw.circle(self.window, wave.color_value(), (top_shelf_x + self.frame_spacing, top_shelf_y), 1)

                (bottom_left_x, bottom_left_y) = self.bottom_left.radial_offset(sample.x, self.max_amplitude, sample.y, self.max_amplitude)
                pygame.draw.circle(self.window, wave.color_value(), (bottom_left_x + self.frame_spacing, bottom_left_y), 1)

            sample = wave.next_sample()
            active_color = Color.BLUE.lerp(wave.color, 0.3)
            active_color = Color.WHITE.lerp(active_color, 0.3)

            # This draws the top shelf
            (top_shelf_x, top_shelf_y) = self.top_shelf.wavelength_offset(wave.phase_index, len(wave.samples), sample.y, self.max_amplitude)
            pygame.draw.circle(self.window, active_color, (top_shelf_x + self.frame_spacing, top_shelf_y), 1)

            # This draws the bottom left
            (bottom_left_x, bottom_left_y) = self.bottom_left.radial_offset(sample.x, self.max_amplitude, sample.y, self.max_amplitude)
            pygame.draw.circle(self.window, active_color, (bottom_left_x + self.frame_spacing, bottom_left_y), 1)


        # Draw the current superposition
        super_sample = self.superposition.samples[self.super_index]

        (top_shelf_x, top_shelf_y) = self.top_shelf.wavelength_offset(self.super_index, len(self.superposition.samples), super_sample.y, self.max_amplitude)
        pygame.draw.circle(self.window, Color.BLUE.value, (top_shelf_x + self.frame_spacing, top_shelf_y), 1)

        (bottom_left_x, bottom_left_y) = self.bottom_left.radial_offset(super_sample.x, self.max_amplitude, super_sample.y, self.max_amplitude)
        pygame.draw.circle(self.window, Color.BLUE.value, (bottom_left_x + self.frame_spacing, bottom_left_y), 1)
        self.super_index = (self.super_index + 1) % len(self.superposition.samples)

        self.draw_bands()


    def draw(self):
        self.window.fill((0, 0, 0))

        self.draw_positions()
        self.draw_shapes()

        pygame.display.update()

    def run(self):
        while self.running:
            self.handle_events()

            if self.update():
                self.draw()
            
            #self.clock.tick(60)