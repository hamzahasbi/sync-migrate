
  

# sync-migrate tool

  

This repos will manage my future data sync from machine to machine including dot files and packages.

  

## Tool Structure

On the root of this project you have a folder containing some utility scripts that will be covered later on and some basic files (to copy paste if you need or just as a reminder to get yours backed up).

  


## How does it work

The files in the root directory serves as a reminder of the files to be backed up from your old laptop .

  

- **shell settings** => themes and plugins for my [ohmyzsh](https://ohmyz.sh/) configuration.

- **.zshrc** => Configuration file for [zsh](https://github.com/ohmyzsh/ohmyzsh/wiki/Installing-ZSH) if you are using zsh as a terminal.

- **personal-zsh-theme.zsh.theme** => This one is a customized (modified by me) zsh theme inspired from [daivasmara](https://github.com/Daivasmara/daivasmara.zsh-theme).

- **settings.zip** => PHPStorm configuration .

  

#### *Scripts folder!*

Inside this folder you have a first script called backup-packages which (as its name indicates) will backup all your installed packages in one folder :

  

> Included package managers are :
>  [brew](https://brew.sh/), [yarn](https://yarnpkg.com/), [npm](https://www.npmjs.com/), [gems](https://rubygems.org/), [composer](https://getcomposer.org/).

  

	chmod a+x ./scripts/backup-packages.sh

	./backup-packages.sh

After running the backup scripts you'll find several *.txt files in packages folder.

Now you're done with your old machine and you can get started with the import on your new machine.

To do that you'll need to run 2 scripts one to reinstall brew packages and the other one for npm/yarn & rubygems .

  

	chmod a+x ./scripts/packages/brew-bulk-install.sh

	chmod a+x ./scripts/packages/packages-reinstall.sh

	./brew-bulk-install.sh

	./packages-reinstall.sh


## Dependencies  
On the new machine you'll need the following tools to be installed.

- [**Brew**](https://brew.sh/)
- [**RubyGems**](https://rubygems.org/)
- [**Yarn**](https://yarnpkg.com/)/[**NPM**](https://www.npmjs.com/)
- [**Composer**](https://getcomposer.org/)
- [**ZSH**](https://github.com/ohmyzsh/ohmyzsh/wiki/Installing-ZSH) (optional)


## ALERT

For composer packages a simpler solution would be to copy your composer.json in your home directory and directly do a composer install (much simpler i guess).

## To contribute
Feel free to submit issues or pull request or enhancement suggestions.
[If you wanna have a chat](mailto:hamza.hasbi@gmail.com).
