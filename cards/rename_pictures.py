import os
# run in the dir of cards
a = os.listdir()

for i in a:
    if i == "s.py":
        continue
    # print(i.lower())
    # run each one individually
    #os.rename(i, i.replace(" ", ""))
    os.rename(i, i.lower())
    # print(i.replace(" ", ""))
    # os.rename(i, i.replace(" ", ""))
