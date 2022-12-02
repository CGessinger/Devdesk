/*
    Vault Database
*/
pub const V_CREATE_TABLE: &str = "CREATE TABLE if not exists Vaults (
    VaultId INTEGER primary key,
    Path TEXT NOT NULL,
    Keep INTEGER DEFAULT 1 NOT NULL,
    ParentVaultId INTEGER NOT NULL,
    CONSTRAINT valid UNIQUE(Path)
)";
pub const V_SET_KEEP: &str = "UPDATE Vaults SET Keep = (?1)";
pub const V_PURGE_KEEP: &str = "DELETE FROM Vaults WHERE Keep = 0";
pub const V_SELECT_ALL_BY_ID: &str = "SELECT * FROM Vaults WHERE VaultId = (?)";
pub const V_SELECT_ALL_BY_PARENT_ID: &str = "SELECT * FROM Vaults WHERE ParentVaultId = (?)";
pub const V_SELECT_ALL_BY_PATH: &str = "SELECT * from Vaults WHERE Path=(?)";
pub const V_UPSERT_OR_KEEP: &str = "INSERT INTO Vaults (Path, ParentVaultId) values (?1, ?2)
    ON CONFLICT DO UPDATE set Keep = 1";

/*
    Project Database
*/
pub const P_CREATE_TABLE: &str = "CREATE TABLE if not exists Projects (
    ProjectId INTEGER primary key,
    Name TEXT not null,
    Path TEXT not null,
    Modified TEXT not null,
    Timestamp STRING DEFAULT CURRENT_TIMESTAMP,
    Keep INTEGER DEFAULT 1 NOT NULL,
    VaultId INTEGER not null,
    FOREIGN KEY(VaultId) REFERENCES Vaults(VaultId),
    CONSTRAINT valid UNIQUE(Path)
)";
pub const P_SET_KEEP: &str = "UPDATE Projects SET Keep = (?1)";
pub const P_PURGE_KEEP: &str = "DELETE FROM Projects WHERE Keep = 0";
pub const P_UPDATE_TIMESTAMP: &str =
    "UPDATE Projects set Timestamp = CURRENT_TIMESTAMP WHERE ProjectId = (?1)";
pub const P_SELECT_ALL_RECENT: &str = "SELECT * FROM Projects ORDER BY Timestamp desc LIMIT (?)";
pub const P_SELECT_ALL_BY_ID: &str = "SELECT * FROM Projects WHERE ProjectId = (?1)";
pub const P_SELECT_ALL_BY_PATH: &str = "SELECT * from Projects WHERE Path=(?)";
pub const P_UPSERT_OR_KEEP: &str =
    "INSERT INTO Projects (Name, Path, Modified, VaultId) values (?1, ?2, ?3, ?4)
    ON CONFLICT DO UPDATE set Keep = 1, Modified = (?3)";
