# 2024-05-20 R. Chris Ancillary

from colors import Color
import pygame

class Frame:
    def __init__ (self, window, x, y, width, height, spacing):
        self.width = width
        self.height = height
        self.x = x
        self.y = y
        self.window = window
        self.frame_color = Color.WHITE.value
        self.midline_color = Color.BLUE.value
        self.spacing = spacing
    
    def wavelength_offset(self, index, n_indices, value, scale): # Index would be the X, value would be Y * scale
        # width / n_indices == width of each index * the idx + x 
        relative_x = index * (self.width / n_indices) + self.x
        relative_y = value * ((self.height // 2) / (scale * 2)) + self.y + (self.height // 2)
        return (relative_x, relative_y)
    
    def bandwidth_offset(self, index, n_indices, amplitude, total_range):
        left_x = self.x + index * (self.width / n_indices)
        relative_offset = (amplitude * (self.height / total_range)) 
        top_y = self.y + self.height - relative_offset + (relative_offset // 2)
        return (left_x, top_y)

    def radial_offset(self, index, n_indices, value, scale): # Index would be the X, value would be Y * scale
        # width / n_indices == width of each index * the idx + x 
        relative_x = index * ((self.width // 2) / (n_indices * 2)) + self.x + (self.width // 2)
        relative_y = value * ((self.height // 2) / (scale * 2)) + self.y + (self.height // 2)
        return (relative_x, relative_y)

    def draw_frame(self):
        left_x = self.x + self.spacing
        right_x = self.x + self.width - self.spacing
        top_y = self.y + self.spacing
        bottom_y = self.y + self.height - self.spacing

        # Top Left to Top Right
        pygame.draw.line(self.window, self.frame_color, (left_x, top_y), (right_x, top_y))
        # Top Right to Bottom Right
        pygame.draw.line(self.window, self.frame_color, (right_x, top_y), (right_x, bottom_y))
        # Bottom Right to Bottom Left
        pygame.draw.line(self.window, self.frame_color, (right_x, bottom_y), (left_x, bottom_y))
        # Bottom Left to Top Left
        pygame.draw.line(self.window, self.frame_color, (left_x, bottom_y), (left_x, top_y))
        
    def draw_cross(self):
        left_x = self.x + self.spacing
        horizontal_y = self.y + (self.height // 2)
        vertical_x = self.x + (self.width // 2)
        top_y = self.y + self.spacing
        
        pygame.draw.line(self.window, self.midline_color, (left_x, horizontal_y), (self.x + self.width - self.spacing, horizontal_y))
        pygame.draw.line(self.window, self.midline_color, (vertical_x, top_y), (vertical_x, self.y + self.height - self.spacing))

    def draw_midline(self):
        horizontal_y = self.y + (self.height // 2)
        pygame.draw.line(self.window, self.midline_color, (self.x, horizontal_y), (self.x + self.width, horizontal_y))
    
    def draw_verticles(self):
        subdivision = self.width // 12

        for i in range(1, 12):
            x = self.x + i * subdivision
            pygame.draw.line(self.window, self.midline_color, (x, self.y), (x, self.y + self.height))