
import pandas as pd
import matplotlib.pyplot as plt

def read_data(file_path):
    data = pd.read_csv(file_path)
    return data
  
def main():
    data = read_data('results.csv')
    # The data is in the form of a pandas dataframe
    # The columns are: <region_size>, <percentage_pages>, <time>
    # All are integers, time is measured in nanoseconds,
    # percentage is a percentage of the total number of pages, such as 1, 10, 20 %.
    # Region size is the size of the region in GB.
    
    # We plot this as a 3D surface plot, with the x-axis being the region size,
    # the y-axis being the percentage of pages, and the z-axis being the time.
    # the color of the points grows in intensity as time is larger.
    
    # Transform nansoseconds to milliseconds
    data['time'] = data['time'] / 1_000_000
    
    fig = plt.figure()
    ax = fig.add_subplot(projection='3d')
    ax.scatter(data['region_size'], data['percentage_pages'], data['time'], c=data['time'], cmap='Reds')
    ax.set_xlabel('Region size (GB)')
    ax.set_ylabel('Percentage of pages (%)')
    ax.set_zlabel('Time (ms)')
    plt.savefig('perf_graph.pdf')
    
    return
    
    
if __name__ == '__main__':
    main()