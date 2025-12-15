print("\033c\033[40;37m\ngive me a file to unmount ? ")
i=input()
f1=open(i,"r")
ff=f1.read()
f1.close()
index=0;
fff=ff.split("|")
names=""
for f in fff:
    
    if index & 1 == 0:
      
      f=f.replace("|","")
      names=f
      print(f)
      print(names)
    else:
      f1=open(names,"w")
      f=f.replace("\xff","|")
      f1.write(f)
      f1.close()
    index=index+1
