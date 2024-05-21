# Copyright 2024 R. Chris Ancillary

import math
from colors import Color

class Wave:
    def __init__(self, wave_type, frequency, amplitude, n_samples=2048, sample_rate=44100, color=Color.WHITE):
        self.wave_type = wave_type
        self.frequency = frequency
        self.amplitude = amplitude
        self.samples = []
        self.sample_count = 0
        self.sample_rate = sample_rate
        self.phase = 0
        self.color = color

        self.generate_wave(n_samples)

    # This is where we implement our variable resolution (we will use proximity) propietary ancillary, tonal synthesis superposition
    def generate_wave(self, n_samples):
        while self.sample_count < n_samples:
            self.samples.append(self.amplitude * math.sin(2 * math.pi * self.frequency * (self.sample_count / self.sample_rate) + self.phase))
            self.sample_count += 1

    def next_sample(self):
        self.phase += 1
        return self.samples[self.phase % len(self.samples)]

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