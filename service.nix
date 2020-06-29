{ config, lib, pkgs, ... }:

let
  orriBackend =
    (import ./backend/Cargo.nix { pkgs = pkgs; }).rootCrate.build;

  orriFrontend =
    import ./frontend/default.nix {
      pkgs = pkgs;
    };

  cfg =
    config.services.orri;

  commonEnvironment = {
    LC_ALL = "en_US.UTF-8";
    LOCALE_ARCHIVE = "${pkgs.glibcLocales}/lib/locale/locale-archive";
    SERVER_FRONTEND_ROOT = "${orriFrontend}";
  };
in
{
  options = {
    services.orri = {
      enable = lib.mkOption {
        type = lib.types.bool;
        default = false;
        description = "Whether to enable orri";
      };

      environment = lib.mkOption {
        type = lib.types.attrs;
        default = {};
        description = "Environment variables for the service";
      };
    };
  };


  config = lib.mkIf cfg.enable {
    # Orri user
    users.extraUsers.orri = {
      createHome = true;
      home = "/home/orri";
      description = "Orri service user";
    };

    # Orri service
    systemd.services.orri = {
      description = "orri backend";
      wantedBy = [ "multi-user.target" ];

      serviceConfig =
        {
          WorkingDirectory = "${orriBackend}";
          ExecStart = "${orriBackend}/bin/orri";
          Restart = "always";
          User = "orri";
        };

      environment = commonEnvironment // cfg.environment;
    };
  };
}
