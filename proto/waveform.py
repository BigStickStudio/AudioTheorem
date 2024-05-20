

class Wave:
    def __init__(self, wave_type, frequency, amplitude):
        self.wave_type = wave_type
        self.frequency = frequency
        self.amplitude = amplitude
        self.phase = 0

    def __str__(self):
        return f"{self.wave_type} wave at {self.frequency} Hz with amplitude {self.amplitude}"


# The waveform is a collection of waves, but in the context of a Time domain and a Frequency domain
class WaveForm:
    def __init__(self):
        self.waves_t = []   # This stores all the waves in the time domain - used for drawing in the Z and X axis, being time and phase
        self.waves_f = []   # This stores all the waves in the frequency domain - used for drawing in the Y axis, being amplitude and phase
