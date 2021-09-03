composer global show -i > ./packages/composer.txt
npm list -g --depth=0 > ./packages/node.txt
yarn global list --depth=0 >> ./packages/node.txt
brew list > brew.txt
ruby -S gem list --local > ./packages/gems.txt

