import datetime
# from time import gmtime, strftime

def take_string(string):
    # s = strftime("%a, %d %b %Y %H:%M:%S", 
    #             gmtime(1627987508.6496193))
    # print(s) 
    x = datetime.datetime.now()
    print(x) 
    print("Calling python function from rust with string: " + string)
