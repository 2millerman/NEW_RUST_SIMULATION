import pygame
import my_rust_lib
import random
import heapq

# Initialize pygame
pygame.init()

# Define some constants
GRID_SIZE = 70
CELL_SIZE = 10
WINDOW_WIDTH = 200
WINDOW_SIZE = 5
CAMERA_VIEW_WIDTH = 200
size = (GRID_SIZE * CELL_SIZE + CAMERA_VIEW_WIDTH, GRID_SIZE * CELL_SIZE)
screen = pygame.display.set_mode(size)


fullscreen = False


clock = pygame.time.Clock()

def find_starting_position(grid):
        while True:
            x = random.randint(0, len(grid[0]) - 1)
            y = random.randint(0, len(grid) - 1)
            if grid[y][x].cell_type == "room" or grid[y][x].cell_type == "hallway":
                return x, y



def a_star_algorithm(grid, start, goal):
    # Define the heuristic function
    def heuristic(a, b):
        return abs(a[0] - b[0]) + abs(a[1] - b[1])

    # Frontier as a priority queue, starting with the start position
    frontier = [(0, start)]
    came_from = {start: None}
    cost_so_far = {start: 0}

    while frontier:
        # Pop the next position from the heap
        priority, current = heapq.heappop(frontier)
        
        # If the current position is the goal, reconstruct the path
        if current == goal:
            path = []
            while current != start:
                path.append(current)
                current = came_from[current]
            path.append(start)
            path.reverse()
            return path

        # Loop through valid neighbors
        for dx, dy in [(1, 0), (0, 1), (-1, 0), (0, -1)]:
            new_x = (current[0] + dx) % len(grid[0])
            new_y = (current[1] + dy) % len(grid)
            neighbor = (new_x, new_y)
            new_cost = cost_so_far[current] + 1

            # Only consider neighbors that are rooms or hallways
            if grid[new_y][new_x].cell_type not in ["room", "hallway"]:
                continue

            # If the new cost is less, update the cost and add the neighbor to the frontier
            if neighbor not in cost_so_far or new_cost < cost_so_far[neighbor]:
                cost_so_far[neighbor] = new_cost
                priority = new_cost + heuristic(goal, neighbor)
                heapq.heappush(frontier, (priority, neighbor))
                came_from[neighbor] = current

    # Return None if no path is found
    return None


class Agent: 
    def __init__(self, start_position, grid, color, num_locations, speed=1):
        self.position = start_position
        self.targets = self.generate_random_targets(grid, num_locations)
        self.color = color
        self.path = []
        self.active = True
        self.speed = speed
        print("Agent speed: ", speed)

    def generate_random_targets(self, grid, num_locations):
        targets = []
        current_room_position = self.position
        for _ in range(num_locations):
            room_found = False
            while not room_found:
                x = random.randint(0, len(grid[0]) - 1)
                y = random.randint(0, len(grid) - 1)
                if grid[y][x].cell_type == "room" and (x, y) != current_room_position:
                    targets.append((x, y))
                    room_found = True
        return targets


    
    
    def move_towards_target(self, grid):
        for _ in range(self.speed):
            if not self.targets:
                return

            if not self.path:  # If there is no path to the current target, generate one using A* algorithm
                target_x, target_y = self.targets[0]
                self.path = a_star_algorithm(grid, self.position, (target_x, target_y))
                
                if self.path is None:
                    # If no path found, remove the target and clear the path
                    self.targets.pop(0)
                    self.path = []
                else:
                    # Remove the first position in the path since it's the agent's current position
                    self.path.pop(0)

            if self.path:
                self.position = self.path.pop(0)  # Move to the next position in the path

                # If the agent has reached the target, remove it from the targets list
                if self.position == self.targets[0]:
                    self.targets.pop(0)
                    self.path = []
                    if not self.targets:
                        self.active = False


# Define a function to draw the grid
def draw_grid():
    for y, row in enumerate(grid):
        for x, cell in enumerate(row):
            if cell.cell_type == "wall":
                pygame.draw.rect(screen, (0, 0, 0), (x * CELL_SIZE, y * CELL_SIZE, CELL_SIZE, CELL_SIZE))
            elif cell.cell_type == "room" or cell.cell_type == "hallway":
                pygame.draw.rect(screen, (255, 255, 255), (x * CELL_SIZE, y * CELL_SIZE, CELL_SIZE, CELL_SIZE))


room_size_factor = float(input("Enter a room size probability between 0 and 1: "))
room_size_factor = max(0.0, min(room_size_factor, 1.0))

hallway_factor = float(input("Enter a hallway factor between 0 and 1: "))
hallway_factor = max(0.0, min(hallway_factor, 1.0))

grid = my_rust_lib.generate_walls(GRID_SIZE, room_size_factor)

# Generate hallways using Rust library
grid = my_rust_lib.generate_hallways(grid, hallway_factor)

num_agents = int(input("Enter the number of agents: "))

# Creating agents
starting_positions = [find_starting_position(grid) for _ in range(num_agents)]
colors = [(random.randint(0, 255), random.randint(0, 255), random.randint(0, 255)) for _ in range(num_agents)]
speeds = [random.randint(1, 2) for _ in range(num_agents)]
agents = [Agent(starting_positions[i], grid, colors[i], 100, speed=speeds[i]) for i in range(num_agents)]



# Main loop
running = True
# rooms = identify_rooms(grid)
# windows = create_windows(rooms)
while running:
    screen.fill((192, 192, 192))  # Fill the screen with white
    draw_grid()
    # draw_windows(windows)
    # draw_camera_view(windows, agents)

    for agent in agents:
        if agent.active:
            print("Agent Position:", agent.position) # Print agent's position every frame
            agent.move_towards_target(grid)

            pygame.draw.circle(screen, agent.color, (agent.position[0] * CELL_SIZE + CELL_SIZE // 2,
                                                     agent.position[1] * CELL_SIZE + CELL_SIZE // 2), CELL_SIZE // 2)
            
    pygame.display.flip()


    for event in pygame.event.get():
        if event.type == pygame.QUIT or (event.type == pygame.KEYDOWN and event.key == pygame.K_ESCAPE):
            running = False
        elif event.type == pygame.KEYDOWN:
            if event.key == pygame.K_f:  # Press 'F' to toggle full-screen
                fullscreen = not fullscreen
                if fullscreen:
                    screen = pygame.display.set_mode((0, 0), pygame.FULLSCREEN)
                else:
                    screen = pygame.display.set_mode(size)
    
    clock.tick(7)

pygame.quit()
