for i in $(cat ./brew.txt)
    do 
        brew install "$i"
    done
