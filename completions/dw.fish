complete -c dw -n "__fish_use_subcommand" -l generate-shell-completion -d 'generate shell completion' -r -f -a "bash zsh powershell fish elvish"
complete -c dw -n "__fish_use_subcommand" -s f -l file -d 'use file' -r
complete -c dw -n "__fish_use_subcommand" -s o -l lang-origin -d 'origin language of the querying text' -r
complete -c dw -n "__fish_use_subcommand" -s t -l lang-target -d 'the language to be translated into' -r
complete -c dw -n "__fish_use_subcommand" -l format -d 'response format' -r -f -a "md ansi"
complete -c dw -n "__fish_use_subcommand" -l server -d 'server mode'
complete -c dw -n "__fish_use_subcommand" -l standalone -d 'standalone client mode'
complete -c dw -n "__fish_use_subcommand" -l lang-code -d 'display all available language codes'
complete -c dw -n "__fish_use_subcommand" -s h -l help -d 'Prints help information'
complete -c dw -n "__fish_use_subcommand" -s V -l version -d 'Prints version information'
