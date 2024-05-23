# Copyright 2024 R. Chris Ancillary

import math
from colors import Color


class Wave:
    def __init__(self, x, y):
        self.x = x  # X represents Phase
        self.y = y  # Y represents Amplitude
    
    def __add__(self, other):
        return Wave(self.x + other.x, self.y + other.y)

class WaveForm:
    def __init__(self, wave_type, idx, frequency, amplitude, n_samples=2048, sample_rate=44100):
        self.idx = idx
        self.wave_type = wave_type
        self.frequency = frequency
        self.amplitude = amplitude
        self.samples = []
        self.sample_count = 0
        self.sample_rate = sample_rate
        self.phase_index = 0
        self.color = Color.get((idx * 3) % 12) # TODO: Figure out how to lerp for more than 12 for more dynamic color
        self.phase = 0

        self.generate_wave(n_samples)

    # This is where we implement our variable resolution (we will use proximity) propietary ancillary, tonal synthesis superposition
    def generate_wave(self, n_samples):
        while self.sample_count < n_samples:
            x = self.amplitude * math.cos(self.phase)
            y = self.amplitude * math.sin(self.phase)
            self.samples.append(Wave(x, y))
            self.phase += 2 * math.pi * self.frequency * (1 / self.sample_rate)
            self.sample_count += 1

    def next_sample(self):
        self.phase_index = self.phase_index + 1 % self.sample_count
        return self.samples[self.phase_index]

    def __str__(self):
        return f"{self.wave_type} wave at {self.frequency} Hz with amplitude {self.amplitude}"
    
    def __lt__(self, other):
        return self.frequency < other.frequency
    
    def __eq__(self, other):
        return self.frequency == other.frequency
    
    def __gt__(self, other):
        return self.frequency > other.frequency
    
    ## TODO: Add safety checking. this is just wreckless.
    def add(self, other):
        self.samples = [x + y for x, y in zip(self.samples, other.samples)]
        return self
    
    def color_value(self):
        return self.color.val()