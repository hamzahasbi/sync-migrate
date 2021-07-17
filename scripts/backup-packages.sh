composer global show -i > composer.txt
npm list -g --depth=0 > node.txt
yarn global list --depth=0 >> node.txt
brew list > brew.txt
ruby -S gem list --local > gems.txt

