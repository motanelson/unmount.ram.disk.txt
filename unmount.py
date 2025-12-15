print("\033c\033[40;37m\ngive me a file to unmount ? ")
i=input()
f1=open(i,"r")
ff=f1.read()
f1.close()
index=0;
fff=ff.split("|")
fff=fff[1:]
names=""
for f in fff:
    if index & 1 == 0:
      names=f
      names.replace("|","")
    else:
      f1=open(names,"w")
      f1.write(f)
      f1.close()
    index=index+1
