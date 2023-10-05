insert into system (uuid, os_name, web_site) VALUES ( 'BEE93347-7F8C-46B5-A385-AD89E7C28825', 'ArchLinux', '');

insert into package_manger (uuid, command, system, install_instruction, update_instruction, delete_instruction) VALUES ('60995545-0424-400A-A293-83F945122BF7', 'sudo pacman --noconfirm', 'BEE93347-7F8C-46B5-A385-AD89E7C28825', '-S', '-Su', '-R');

insert into software (uuid, long_name, short_name, developer, web_site) VALUES ( '96F5479F-BC17-420C-A3D9-3F3B5DF63A70', 'Neovim', 'neovim', '', '');

insert into packages (uuid, package_name, software, package_manger) VALUES ('99B6090D-49F3-4F4F-A7DB-58161A8E3065', 'neovim', '96F5479F-BC17-420C-A3D9-3F3B5DF63A70', '60995545-0424-400A-A293-83F945122BF7')
