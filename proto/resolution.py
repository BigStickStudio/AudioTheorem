from enum import Enum

ONE_CENT = 21 / 1200
ONE_STEP = ONE_CENT * 10

# The Concept here is that we increase resolution while narrowing the bands shown
class Resolution(Enum):
    SEMITONE = 0        # This is the resolution where we have 12 bands per octave == 144 bands
    SUBTONE = 1         # This is the resolution where we have 5 bands per pitch class == 60 bands per octave == 
    SUBSTEP = 2         # This is the resolution where we have 10 bands per pitch class == 120 bands per octave == 1440 bands
    CENT = 3            # This is the resolution where we have 100 bands per semitone == 1200 bands per octave == 14400 bands
    
    # Returns the total number of subdivisions across 12 octaves (from ~8hz to ~32khz)
    def subdivisions(self):
        if self == Resolution.SEMITONE:
            return 144
        elif self == Resolution.SUBTONE:
            return 720
        elif self == Resolution.SUBSTEP:
            return 1440
        elif self == Resolution.CENT:
            return 14400
        else:
            return 0