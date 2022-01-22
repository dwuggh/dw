complete -c dw -l generate-shell-completion -d 'generate shell completion' -r -f -a "{bash	,elvish	,fish	,powershell	,zsh	}"
complete -c dw -s f -l file -d 'use file' -r
complete -c dw -s o -l lang-origin -d 'origin language of the querying text' -r
complete -c dw -s t -l lang-target -d 'the language to be translated into' -r
complete -c dw -l format -d 'response format' -r -f -a "{md	,ansi	}"
complete -c dw -s h -l help -d 'Print help information'
complete -c dw -s V -l version -d 'Print version information'
complete -c dw -l server -d 'server mode'
complete -c dw -l standalone -d 'standalone client mode'
complete -c dw -l lang-code -d 'display all available language codes'
