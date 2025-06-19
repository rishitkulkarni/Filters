import pandas as pd 
import matplotlib.pyplot as plt 

raw = pd.read_csv("C:/Users/ADMIN/Downloads/lollslls.csv")
filtered = pd.read_csv("filteredPressure.csv")

plt.plot(raw["time"],raw["pressure"], label="Raw Pressure")
plt.plot (filtered["time"],filtered["filteredPressure"], label="Filtered Pressure")

plt.xlabel("Time (s)")
plt.ylabel("Pressure (Pa)")     
plt.title("FIR Filter")
plt.legend()
plt.grid()
plt.tight_layout()
plt.show()