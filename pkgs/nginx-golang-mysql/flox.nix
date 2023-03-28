{
  # flox environment configuration
  #
  # flox.nix options: https://floxdev.com/docs/reference/flox-nix-config
  # Getting started with flox: https://floxdev.com/docs
  # Get help: https://discourse.floxdev.com
  #
  # Happy hacking!
  packages.nixpkgs-flox.nginx = {};
  packages.nixpkgs-flox.go = {};
  packages.nixpkgs-flox.mysql = {};
  packages.nixpkgs-flox.figlet = {};

  shell.hook = ''
  figlet dev

  MYSQL_HOME=$PWD/.mysql
  MYSQL_DATADIR=$MYSQL_HOME/data
  export MYSQL_UNIX_PORT=$MYSQL_HOME/mysql.sock
  MYSQL_PID_FILE=$MYSQL_HOME/mysql.pid
  alias mysql='mysql -u root'

  # kill any running mysql instance
  if [ -f $MYSQL_PID_FILE ]; then
    kill $(cat $MYSQL_PID_FILE)
  fi

  if [ ! -d "$MYSQL_HOME" ]; then
    # Make sure to use normal authentication method otherwise we can only
    # connect with unix account. But users do not actually exists in nix.
    mysql_install_db --auth-root-authentication-method=normal \
      --datadir=$MYSQL_DATADIR \
      --pid-file=$MYSQL_PID_FILE
  fi

  # Starts the daemon
  #mysqld --datadir=$MYSQL_DATADIR --pid-file=$MYSQL_PID_FILE \
  #  --socket=$MYSQL_UNIX_PORT 2> $MYSQL_HOME/mysql.log &
  #MYSQL_PID=$!

  #finish()
  #{
  #  mysqladmin -u root --socket=$MYSQL_UNIX_PORT shutdown
  #  kill $MYSQL_PID
  #}
  #trap finish EXIT
  '';

  environmentVariables.MYSQL_DATABASE = "flox";
  environmentVariables.MYSQL_ROOT_PASSWORD = "db-q5n2g";
  environmentVariables.MYSQL_HOST = "localhost";
  environmentVariables.MYSQL_TEST_DATABASE = "test";
  environmentVariables.MYSQL_USER = "flox";
  environmentVariables.MYSQL_PASSWORD = "db-q5n2g";
}
