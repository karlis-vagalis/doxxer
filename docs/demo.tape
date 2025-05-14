Set Shell fish
Output demo.gif
Env fish_autosuggestion_enabled "0"

Sleep 500ms

Type "doxxer help"
Sleep 500ms
Enter
Sleep 1.5s

Hide
Type "clear && cd .."
Enter
Show

Type "doxxer current"
Sleep 500ms
Enter
Sleep 1.5s

Type "doxxer next"
Sleep 500ms
Enter
Sleep 1.5s

Type `doxxer next --pre-template "post.{distance}" --build-template ""`
Sleep 500ms
Enter
Sleep 1.5s

Type "doxxer next --strategy patch"
Sleep 500ms
Enter
Sleep 1.5s

Type "doxxer next -s minor"
Sleep 500ms
Enter
Sleep 1.5s

Type "doxxer next -s minor minor"
Sleep 500ms
Enter
Sleep 1.5s

Type `doxxer -p "release-" next -s major`
Sleep 500ms
Enter
Sleep 1.5s

Type `doxxer -p "" next -s major`
Sleep 500ms
Enter
Sleep 1.5s

Sleep 1s