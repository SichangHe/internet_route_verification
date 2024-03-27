use crate::{Report::*, ReportItem::*, *};
use dashmap::DashMap;
use maplit::hashmap;

use super::*;

const IR: &str = r#"{"aut_nums":{
"196763":{"body":"","n_import":14,"n_export":14,"imports":{"ipv4":{"unicast":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":9063}}}}],"mp_filter":"Any"},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":60351}}}}],"mp_filter":{"AsNum":[60351,"NoOp"]}}]}},"exports":{"ipv4":{"unicast":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":9063}}}}],"mp_filter":{"AsNum":[196763,"NoOp"]}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":60351}}}}],"mp_filter":"Any"}]}}},
"2914":{"body":"","n_import":1,"n_export":1,"imports":{"any":{"any":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":50472}}}}],"mp_filter":{"AsSet":["AS-CHAOS","NoOp"]}}]}},"exports":{"any":{"any":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Set":"AS-ANY"}}}}],"mp_filter":{"AsSet":["AS2914:AS-GLOBAL","NoOp"]}}]},"ipv6":{"unicast":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Set":"AS-ANY"}}}}],"mp_filter":{"AsSet":["AS2914:AS-GLOBAL-v6","NoOp"]}}]}}},
"9583":{"body":"","n_import":1,"n_export":1,"imports":{"any":{"any":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":701}}},"actions":{"pref":"20"}}],"mp_filter":"Any"}]}},"exports":{"any":{"any":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":4637}}}}],"mp_filter":{"AsNum":[9583,"NoOp"]}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":701}}}}],"mp_filter":{"AsNum":[9583,"NoOp"]}}]}}},
"18106":{"body":"","n_import":1,"n_export":1,"imports":{"any":{"any":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":6939}}},"actions":{"pref":"100"}}],"mp_filter":"Any"},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":174}}},"actions":{"pref":"100"}}],"mp_filter":"Any"},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":4657}}},"actions":{"pref":"100"}}],"mp_filter":"Any"},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":2914}}},"actions":{"pref":"100"}}],"mp_filter":"Any"}]}},"exports":{"any":{"any":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":6939}}}}],"mp_filter":{"Or":{"left":{"AsNum":[18106,"NoOp"]},"right":{"AsSet":["AS18106:AS-TRANSIT","NoOp"]}}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":174}}}}],"mp_filter":{"Or":{"left":{"AsNum":[18106,"NoOp"]},"right":{"AsSet":["AS18106:AS-TRANSIT","NoOp"]}}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":4657}}}}],"mp_filter":{"Or":{"left":{"AsNum":[18106,"NoOp"]},"right":{"AsSet":["AS18106:AS-TRANSIT","NoOp"]}}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":2914}}}}],"mp_filter":{"Or":{"left":{"AsNum":[18106,"NoOp"]},"right":{"AsSet":["AS18106:AS-TRANSIT","NoOp"]}}}}]}}},
"196844":{"body":"","n_import":1,"n_export":1,"imports":{"any":{"any":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Set":"AS-AMS-IX-PEERS"}}}}],"mp_filter":"Any"},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":29414}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-Bialystok-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":12618}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-Bydgoszcz-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":25084}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-Czestochowa-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":15396}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-ICM-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":30778}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-Kielce-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":28797}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-Koszalin-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":8323}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-Krakow-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":16283}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-LODMAN-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":12346}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-Lublin-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":8308}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-NASK-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":21064}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-Olsztyn-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":25584}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-Opole-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":8364}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-POZMAN-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":34604}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-Pulawy-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":16263}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-Radom$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":39873}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-Rzeszow-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":15744}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-Slask-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":13119}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-Szczecin-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":12831}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-TASK-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":35686}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-Torun-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":15851}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-Wroclaw-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":13065}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-Zielona_Gora-COM$"}}]}},"exports":{"any":{"any":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":29414}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":12618}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":25084}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":15396}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":30778}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":28797}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":8323}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":16283}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":12346}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":8308}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":21064}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":25584}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":8364}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":34604}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":16263}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":39873}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":15744}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":13119}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":12831}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":35686}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":15851}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":13065}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":21021}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}}]}}},
"20912":{"body":"","n_import":761,"n_export":774,"imports":{"ipv4":{"unicast":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":6939}}}}],"mp_filter":{"AsNum":[6939,"NoOp"]}}]}},"exports":{}},
"33549":{"body":"","n_import":0,"n_export":1,"imports":{},"exports":{"any":{"any":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":"Any"}}}],"mp_filter":{"AsSet":["AS33549:AS-ALL","NoOp"]}}]}}},
"6939":{"body":"","n_import":1,"n_export":2,"imports":{"ipv4":{"unicast":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":"Any"}}}],"mp_filter":"Any"}]}},"exports":{"ipv4":{"unicast":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":"Any"}}}],"mp_filter":{"AsSet":["AS-HURRICANE","NoOp"]}}]},"ipv6":{"any":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":"Any"}}}],"mp_filter":{"AsSet":["AS-HURRICANEv6","NoOp"]}}]}}},
"9063":{"body":"","n_import":32,"n_export":32,"imports":{"ipv4":{"unicast":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":196763}}},"actions":{"pref":"1000"}}],"mp_filter":{"AsNum":[196763,"NoOp"]}}]}},"exports":{}}
},"as_sets":{
"AS33549:AS-ALL":{"body":"","members":[33549],"set_members":[],"is_any":false}
},"route_sets":{},"peering_sets":{},"filter_sets":{},"as_routes":{
"196763":["5.22.144.0/21","5.22.144.0/22","5.22.144.0/23","5.22.144.0/24","5.22.145.0/24","5.22.146.0/23","5.22.148.0/22","78.109.56.0/21","93.190.236.0/23","93.190.238.0/23","109.234.104.0/21","178.254.63.0/24","185.21.164.0/22","185.33.180.0/22","193.46.215.0/24","2a00:18e0::/32","2a00:18e0::/48","2a00:18e0:1::/48","2a00:18e0:6::/48","2a00:18e0:7::/48","2a00:8140:1000::/36"],
"33549":["64.25.108.0/22","64.25.108.0/23","64.25.108.0/24","64.25.109.0/24","64.25.110.0/23","64.25.110.0/24","64.25.111.0/24","64.191.44.0/23","64.191.44.0/24","64.191.45.0/24","104.37.148.0/22","104.37.148.0/24","104.37.149.0/24","104.37.150.0/24","104.37.151.0/24","135.84.136.0/22","135.84.136.0/24","135.84.137.0/24","135.84.138.0/24","135.84.139.0/24","158.106.98.0/24","199.247.206.0/23","202.170.192.0/22","202.170.192.0/24","202.170.193.0/24","202.170.194.0/24","202.170.195.0/24","204.191.218.0/23","207.228.103.0/24","2604:6440::/32","2604:6440::/48"],
"6939":["5.180.83.0/24","23.139.56.0/24","23.142.192.0/24","23.145.128.0/24","23.146.80.0/24","23.164.160.0/24","23.171.48.0/24","23.175.160.0/24","27.50.32.0/21","27.50.36.0/24","38.87.144.0/23","45.12.80.0/24","45.12.83.0/24","45.67.196.0/22","46.29.24.0/22","50.114.39.0/24","52.129.12.0/23","64.7.224.0/21","64.7.232.0/23","64.7.234.0/24","64.7.236.0/22","64.32.44.0/24","64.62.128.0/17","64.62.128.0/18","64.62.184.0/24","64.62.245.0/24","64.71.128.0/18","64.71.136.0/24","65.19.128.0/18","65.19.128.0/20","65.19.186.0/24","65.49.0.0/17","65.49.0.0/18","65.49.2.0/24","65.49.14.0/24","65.49.68.0/24","65.49.104.0/22","65.49.108.0/22","66.119.119.0/24","66.160.128.0/18","66.160.128.0/20","66.160.192.0/20","66.207.160.0/20","66.220.0.0/19","66.220.0.0/20","66.220.16.0/20","67.21.38.0/23","67.43.48.0/20","67.59.106.0/24","72.14.64.0/24","72.14.65.0/24","72.14.66.0/24","72.14.67.0/24","72.14.72.0/24","72.14.75.0/24","72.14.89.0/24","72.14.90.0/24","72.52.64.0/18","72.52.64.0/19","72.52.71.0/24","72.52.92.0/24","72.52.97.0/24","74.82.0.0/18","74.82.22.0/23","74.82.46.0/24","74.82.48.0/22","74.121.8.0/24","74.121.10.0/24","74.121.104.0/22","74.122.152.0/21","76.78.6.0/23","77.241.72.0/22","88.216.128.0/21","89.116.126.0/24","91.188.252.0/24","91.242.81.0/24","98.159.120.0/21","102.129.208.0/24","102.165.57.0/24","103.6.216.0/22","103.83.124.0/24","103.100.138.0/24","103.120.67.0/24","103.120.207.0/24","103.138.32.0/24","103.139.90.0/24","103.140.202.0/23","103.140.202.0/24","103.140.203.0/24","103.148.243.0/24","103.153.102.0/23","103.176.58.0/24","103.176.59.0/24","103.195.65.0/24","103.207.71.0/24","103.253.24.0/22","103.253.24.0/24","103.253.25.0/24","103.253.26.0/24","103.253.27.0/24","104.36.120.0/22","104.164.206.0/24","104.165.12.0/24","104.165.72.0/24","104.165.195.0/24","104.194.4.0/24","104.194.216.0/23","104.234.45.0/24","104.234.153.0/24","104.254.152.0/21","104.255.240.0/21","107.164.22.0/24","107.164.24.0/24","107.164.86.0/24","107.164.139.0/24","107.164.173.0/24","107.164.223.0/24","107.165.169.0/24","107.165.189.0/24","107.165.197.0/24","107.165.212.0/24","107.165.215.0/24","107.165.216.0/24","107.165.224.0/24","107.186.4.0/24","107.186.9.0/24","107.186.13.0/24","107.186.20.0/24","107.186.23.0/24","107.186.25.0/24","107.186.30.0/24","107.186.32.0/24","107.186.56.0/24","107.186.63.0/24","107.186.64.0/24","107.186.79.0/24","107.186.91.0/24","107.186.152.0/24","107.186.193.0/24","107.186.203.0/24","107.186.224.0/24","107.186.234.0/24","107.187.38.0/24","107.187.43.0/24","107.187.101.0/24","107.187.102.0/24","107.187.104.0/24","107.187.108.0/24","107.187.111.0/24","108.165.70.0/24","108.165.72.0/24","108.165.108.0/23","108.165.120.0/22","108.165.208.0/24","108.165.238.0/24","108.165.239.0/24","109.122.211.0/24","128.254.252.0/22","134.195.36.0/24","134.195.37.0/24","134.195.38.0/24","134.195.39.0/24","136.0.37.0/24","136.0.50.0/24","136.0.73.0/24","139.28.212.0/22","139.28.240.0/22","139.177.158.0/24","141.193.188.0/23","142.202.65.0/24","142.202.216.0/22","148.51.0.0/16","148.51.0.0/17","154.16.45.0/24","154.16.231.0/24","155.254.225.0/24","155.254.226.0/24","155.254.227.0/24","158.222.23.0/24","161.129.140.0/22","162.247.75.0/24","162.249.152.0/23","162.249.154.0/23","162.254.80.0/22","166.0.182.0/23","166.0.190.0/23","166.0.195.0/24","166.0.236.0/23","167.136.239.0/24","168.245.149.0/24","170.199.208.0/23","172.111.23.0/24","176.53.156.0/22","181.214.129.0/24","181.214.183.0/24","181.214.237.0/24","181.215.23.0/24","184.75.240.0/21","184.104.0.0/15","184.104.0.0/17","184.104.176.0/21","184.104.190.0/23","184.104.200.0/21","184.104.208.0/20","184.104.224.0/21","184.104.232.0/22","184.104.236.0/22","184.105.7.0/24","184.105.8.0/21","184.105.10.0/24","184.105.16.0/20","184.105.32.0/20","184.105.48.0/20","184.105.60.0/23","184.105.60.0/24","184.105.61.0/24","184.105.62.0/24","184.105.88.0/21","184.105.100.0/22","184.105.180.0/24","184.105.195.0/24","184.105.248.0/21","185.101.97.0/24","185.101.98.0/24","185.115.84.0/22","185.115.84.0/24","185.130.47.0/24","185.149.68.0/24","185.149.69.0/24","185.149.70.0/24","185.204.103.0/24","191.101.122.0/24","192.88.99.0/24","192.132.94.0/24","192.136.112.0/24","192.190.255.0/24","193.32.204.0/22","193.233.18.0/24","194.15.115.0/24","198.102.8.0/24","198.102.73.0/24","198.102.244.0/24","199.4.150.0/24","199.83.123.0/24","199.88.158.0/24","199.192.144.0/22","199.233.90.0/24","199.245.105.0/24","204.13.226.0/23","204.14.80.0/22","204.62.157.0/24","204.238.49.0/24","205.159.239.0/24","207.126.64.0/19","208.65.255.0/24","208.75.96.0/21","208.79.140.0/22","208.80.92.0/24","208.80.93.0/24","208.80.94.0/24","208.80.95.0/24","208.86.35.0/24","208.101.226.0/24","208.123.222.0/24","209.51.160.0/19","209.51.170.0/24","209.135.0.0/19","209.142.71.0/24","209.150.160.0/19","209.160.106.0/23","212.87.196.0/22","213.52.131.0/24","216.66.0.0/19","216.66.12.0/24","216.66.16.0/24","216.66.18.0/24","216.66.19.0/24","216.66.20.0/24","216.66.21.0/24","216.66.32.0/19","216.66.32.0/22","216.66.64.0/19","216.66.72.0/21","216.66.74.0/23","216.66.80.0/20","216.74.121.0/24","216.99.220.0/23","216.99.221.0/24","216.151.156.0/23","216.177.134.0/24","216.218.128.0/17","216.218.221.0/24","216.218.232.0/24","216.218.252.0/24","216.218.253.0/24","216.224.64.0/19","216.224.64.0/21","216.229.96.0/20","216.235.85.0/24","216.252.162.0/24","2001:470::/32","2001:470:1a::/48","2001:470:1f13::/48","2001:df0:3a80::/48","2001:df2:7900::/48","2001:49e8::/32","2002::/16","2400:7a00::/32","2401:3740:374::/48","2401:3740:375::/48","2404:bb40::/32","2600:7000::/24","2602:fb1b:2::/48","2602:fbad::/40","2602:fbad::/45","2602:fbad:10::/45","2602:fbc5::/48","2602:fc71:fff::/48","2602:fcd7::/36","2602:fd3f:2::/48","2602:fd6a::/36","2602:fd9b::/36","2602:feca::/36","2602:ff06:725::/48","2604:a100:100::/48","2604:a100:200::/48","2604:c800:ffff::/48","2605:4c0::/32","2605:3ac0:1000::/36","2606:7b00:3fff::/48","2620:0:50c0::/48","2a07:e00::/32","2a07:e00:c::/48","2a07:e03::/32","2a07:54c2:b00b::/48","2a09:2580::/29","2a09:2780::/29","2a09:3880::/29","2a09:3b80::/29","2a09:3d80::/29","2a09:e500::/29","2a09:f480::/29","2a09:fa80::/29","2a0d:d540::/29","2a0d:d640::/29","2a10:7d40::/29","2a10:cc40:112::/48"],
"60351":["37.77.200.0/24"]
}}"#;
const LINES: [&str; 3] = [
    "TABLE_DUMP2|1687212000|B|147.28.7.1|3130|1.6.165.0/24|3130 1239 2914 9583|IGP|147.28.7.1|0|0|1239:321 1239:1000 1239:1010|NAG||",
    "TABLE_DUMP2|1687212015|B|212.66.96.126|20912|104.37.148.0/24|20912 6939 33549|IGP|212.66.96.126|0|0|20912:65016|NAG|33549 10.12.255.1|",
    "TABLE_DUMP2|1687212002|B|140.192.8.16|20130|37.77.200.0/24|9063 196763 60351|IGP|140.192.8.16|0|0||NAG||",
];

#[test]
fn err_only_checks() -> Result<()> {
    let query = query()?;
    let db = as_relationship_db()?;
    for (expected, line) in expected_err_only_reports().into_iter().zip(LINES) {
        let mut compare = Compare::with_line_dump(line)?;
        compare.verbosity = Verbosity {
            stop_at_first: false,
            per_filter_err: true,
            all_err: true,
            ..Verbosity::default()
        };
        let actual = compare.check_with_relationship(&query, &db);
        assert_eq!(expected, actual);
    }
    Ok(())
}

fn expected_err_only_reports() -> [Vec<Report>; 3] {
    [
        vec![
            BadExport {
                from: 9583,
                to: 2914,
                items: vec![MatchRemoteAsNum(4637), MatchRemoteAsNum(701)],
            },
            BadImport {
                from: 9583,
                to: 2914,
                items: vec![MatchRemoteAsNum(50472)],
            },
        ],
        vec![BadImport {
            from: 6939,
            to: 20912,
            items: vec![MatchFilterAsNum(6939, RangeOperator::NoOp), MatchFilter],
        }],
        vec![
            BadExport {
                from: 196763,
                to: 9063,
                items: vec![
                    MatchFilterAsNum(196763, RangeOperator::NoOp),
                    MatchFilter,
                    MatchRemoteAsNum(60351),
                ],
            },
            BadImport {
                from: 196763,
                to: 9063,
                items: vec![MatchFilterAsNum(196763, RangeOperator::NoOp), MatchFilter],
            },
        ],
    ]
}

#[test]
fn ok_skip_checks() -> Result<()> {
    let query = query()?;
    let db = as_relationship_db()?;
    for (expected, line) in expected_ok_skip_checks().into_iter().zip(LINES) {
        let mut compare = Compare::with_line_dump(line)?;
        compare.verbosity = Verbosity::minimum_all();
        let actual = compare.check_with_relationship(&query, &db);
        assert_eq!(expected, actual);
    }
    Ok(())
}

fn expected_ok_skip_checks() -> [Vec<Report>; 3] {
    [
        vec![
            MehExport {
                from: 9583,
                to: 2914,
                items: vec![SpecUphill],
            },
            MehImport {
                from: 9583,
                to: 2914,
                items: vec![SpecUphill],
            },
            UnrecExport {
                from: 2914,
                to: 1239,
                items: vec![
                    UnrecordedAsSet("AS-ANY".into()),
                    UnrecordedAsSetRoute("AS2914:AS-GLOBAL".into()),
                ],
            },
            UnrecImport {
                from: 2914,
                to: 1239,
                items: vec![UnrecordedAutNum(1239)],
            },
            UnrecExport {
                from: 1239,
                to: 3130,
                items: vec![UnrecordedAutNum(1239)],
            },
            UnrecImport {
                from: 1239,
                to: 3130,
                items: vec![UnrecordedAutNum(3130)],
            },
        ],
        vec![
            OkExport {
                from: 33549,
                to: 6939,
            },
            OkImport {
                from: 33549,
                to: 6939,
            },
            UnrecExport {
                from: 6939,
                to: 20912,
                items: vec![UnrecordedAsSetRoute("AS-HURRICANE".into())],
            },
            MehImport {
                from: 6939,
                to: 20912,
                items: vec![SpecImportFromNeighbor],
            },
        ],
        vec![
            UnrecExport {
                from: 60351,
                to: 196763,
                items: vec![UnrecordedAutNum(60351)],
            },
            OkImport {
                from: 60351,
                to: 196763,
            },
            MehExport {
                from: 196763,
                to: 9063,
                items: vec![SpecExportCustomers],
            },
            MehImport {
                from: 196763,
                to: 9063,
                items: vec![SpecImportCustomer],
            },
        ],
    ]
}

const DB_FILE: &str = "1239|3130|-1
1239|2914|0
2914|9583|-1
2914|4096|-1
1299|33549|-1
6939|20912|0
196763|60351|-1
9063|196763|-1
";

#[test]
fn stats() -> Result<()> {
    let query = query()?;
    let db = as_relationship_db()?;
    for (expected, line) in expected_stats().into_iter().zip(LINES) {
        let map = DashMap::new();
        let mut compare = Compare::with_line_dump(line)?;
        compare.as_stats(&query, &db, &map);
        let actual = HashMap::from_iter(map.into_iter());
        assert_eq!(expected, actual);
    }
    Ok(())
}

fn expected_stats() -> [HashMap<u32, RouteStats<u64>>; 1] {
    [hashmap! {
        3130 => RouteStats { import_unrec: 1, unrec_aut_num: 1, ..Default::default() },
        1239 => RouteStats { import_unrec: 1, export_unrec: 1, unrec_aut_num: 2, ..Default::default() },
        9583 => RouteStats { export_meh: 1, spec_uphill: 1, ..Default::default() },
        2914 => RouteStats { export_unrec: 1, import_meh: 1, unrec_as_set: 1, spec_uphill: 1, ..Default::default() }
    }]
}

pub fn ir() -> Result<Ir> {
    Ok(serde_json::from_str(IR)?)
}

pub fn query() -> Result<QueryIr> {
    Ok(QueryIr::from_ir_and_as_relationship(
        ir()?,
        &as_relationship_db()?,
    ))
}

pub fn as_relationship_db() -> Result<AsRelDb> {
    AsRelDb::from_lines(DB_FILE.lines())
}
