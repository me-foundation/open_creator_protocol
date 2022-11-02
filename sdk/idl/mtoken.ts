export type Mtoken = {
  "version": "0.1.0",
  "name": "mtoken",
  "instructions": [
    {
      "name": "initMintManager",
      "accounts": [
        {
          "name": "mintManager",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "ruleset",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "collector",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "updateMintManager",
      "accounts": [
        {
          "name": "mintManager",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "ruleset",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "collector",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "ix",
          "type": {
            "defined": "UpdateMintManagerIx"
          }
        }
      ]
    },
    {
      "name": "initRuleset",
      "accounts": [
        {
          "name": "ruleset",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "ix",
          "type": {
            "defined": "InitRulesetIx"
          }
        }
      ]
    },
    {
      "name": "updateRuleset",
      "accounts": [
        {
          "name": "ruleset",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "ix",
          "type": {
            "defined": "UpdateRulesetIx"
          }
        }
      ]
    },
    {
      "name": "initMint",
      "accounts": [
        {
          "name": "mintManager",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "ruleset",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "targetTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "target",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "collector",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "initAccount",
      "accounts": [
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenAccountOwner",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "approve",
      "accounts": [
        {
          "name": "mintManager",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "holderTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "holder",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "delegate",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "revoke",
      "accounts": [
        {
          "name": "mintManager",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "holderTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "holder",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "burn",
      "accounts": [
        {
          "name": "mintManager",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "holderTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "holder",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "close",
      "accounts": [
        {
          "name": "mintManager",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "owner",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "transfer",
      "accounts": [
        {
          "name": "mintManager",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "ruleset",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "from",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "to",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "instructions",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "preTransfer",
      "accounts": [
        {
          "name": "accountBalances",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "instructions",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "postTransfer",
      "accounts": [
        {
          "name": "accountBalances",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "collector",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "instructions",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "mintManager",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "version",
            "type": "u8"
          },
          {
            "name": "mint",
            "type": "publicKey"
          },
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "ruleset",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "ruleset",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "version",
            "type": "u8"
          },
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "collector",
            "type": "publicKey"
          },
          {
            "name": "checkSellerFeeBasisPoints",
            "type": "bool"
          },
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "disallowedAddresses",
            "type": {
              "vec": "publicKey"
            }
          },
          {
            "name": "allowedPrograms",
            "type": {
              "vec": "publicKey"
            }
          }
        ]
      }
    },
    {
      "name": "accountBalances",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "balances",
            "type": {
              "vec": {
                "defined": "AccountBalance"
              }
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "UpdateMintManagerIx",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "InitRulesetIx",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "checkSellerFeeBasisPoints",
            "type": "bool"
          },
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "collector",
            "type": "publicKey"
          },
          {
            "name": "disallowedAddresses",
            "type": {
              "vec": "publicKey"
            }
          },
          {
            "name": "allowedPrograms",
            "type": {
              "vec": "publicKey"
            }
          }
        ]
      }
    },
    {
      "name": "UpdateRulesetIx",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "collector",
            "type": "publicKey"
          },
          {
            "name": "checkSellerFeeBasisPoints",
            "type": "bool"
          },
          {
            "name": "disallowedAddresses",
            "type": {
              "vec": "publicKey"
            }
          },
          {
            "name": "allowedPrograms",
            "type": {
              "vec": "publicKey"
            }
          }
        ]
      }
    },
    {
      "name": "AccountBalance",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "address",
            "type": "publicKey"
          },
          {
            "name": "mint",
            "type": "publicKey"
          },
          {
            "name": "size",
            "type": "u64"
          },
          {
            "name": "balance",
            "type": "u64"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidMint",
      "msg": "Invalid mint"
    },
    {
      "code": 6001,
      "name": "InvalidCollector",
      "msg": "Invalid collector address"
    },
    {
      "code": 6002,
      "name": "InvalidAuthority",
      "msg": "Invalid authority address"
    },
    {
      "code": 6003,
      "name": "InvalidMintManager",
      "msg": "Invalid mint manager"
    },
    {
      "code": 6004,
      "name": "InvlaidHolderTokenAccount",
      "msg": "Invalid holder token account"
    },
    {
      "code": 6005,
      "name": "InvalidTargetTokenAccount",
      "msg": "Invalid target token account"
    },
    {
      "code": 6006,
      "name": "InvalidCloseTokenAccount",
      "msg": "Invalid token account to close"
    },
    {
      "code": 6007,
      "name": "InvalidHolderTokenAccount",
      "msg": "Invalid holder token account"
    },
    {
      "code": 6008,
      "name": "InvalidRuleset",
      "msg": "Invalid ruleset"
    },
    {
      "code": 6009,
      "name": "InvalidPreTransferInstruction",
      "msg": "Invalid pre transfer instruction"
    },
    {
      "code": 6010,
      "name": "InvalidPostTransferInstruction",
      "msg": "Invalid post transfer instruction"
    },
    {
      "code": 6011,
      "name": "ProgramDisallowed",
      "msg": "Disallowed program included in transfer"
    },
    {
      "code": 6012,
      "name": "ProgramNotAllowed",
      "msg": "Program not allowed in allowed programs to transfer"
    },
    {
      "code": 6013,
      "name": "UnknownAccount",
      "msg": "Unknown account found in instruction"
    },
    {
      "code": 6014,
      "name": "AccountNotFound",
      "msg": "Account not found in instruction"
    }
  ]
};

export const IDL: Mtoken = {
  "version": "0.1.0",
  "name": "mtoken",
  "instructions": [
    {
      "name": "initMintManager",
      "accounts": [
        {
          "name": "mintManager",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "ruleset",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "collector",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "updateMintManager",
      "accounts": [
        {
          "name": "mintManager",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "ruleset",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "collector",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "ix",
          "type": {
            "defined": "UpdateMintManagerIx"
          }
        }
      ]
    },
    {
      "name": "initRuleset",
      "accounts": [
        {
          "name": "ruleset",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "ix",
          "type": {
            "defined": "InitRulesetIx"
          }
        }
      ]
    },
    {
      "name": "updateRuleset",
      "accounts": [
        {
          "name": "ruleset",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "ix",
          "type": {
            "defined": "UpdateRulesetIx"
          }
        }
      ]
    },
    {
      "name": "initMint",
      "accounts": [
        {
          "name": "mintManager",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "ruleset",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "targetTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "target",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "collector",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "initAccount",
      "accounts": [
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenAccountOwner",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "approve",
      "accounts": [
        {
          "name": "mintManager",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "holderTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "holder",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "delegate",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "revoke",
      "accounts": [
        {
          "name": "mintManager",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "holderTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "holder",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "burn",
      "accounts": [
        {
          "name": "mintManager",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "holderTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "holder",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "close",
      "accounts": [
        {
          "name": "mintManager",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "owner",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "transfer",
      "accounts": [
        {
          "name": "mintManager",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "ruleset",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "from",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "to",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "instructions",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "preTransfer",
      "accounts": [
        {
          "name": "accountBalances",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "instructions",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "postTransfer",
      "accounts": [
        {
          "name": "accountBalances",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "collector",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "instructions",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "mintManager",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "version",
            "type": "u8"
          },
          {
            "name": "mint",
            "type": "publicKey"
          },
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "ruleset",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "ruleset",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "version",
            "type": "u8"
          },
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "collector",
            "type": "publicKey"
          },
          {
            "name": "checkSellerFeeBasisPoints",
            "type": "bool"
          },
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "disallowedAddresses",
            "type": {
              "vec": "publicKey"
            }
          },
          {
            "name": "allowedPrograms",
            "type": {
              "vec": "publicKey"
            }
          }
        ]
      }
    },
    {
      "name": "accountBalances",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "balances",
            "type": {
              "vec": {
                "defined": "AccountBalance"
              }
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "UpdateMintManagerIx",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "InitRulesetIx",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "checkSellerFeeBasisPoints",
            "type": "bool"
          },
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "collector",
            "type": "publicKey"
          },
          {
            "name": "disallowedAddresses",
            "type": {
              "vec": "publicKey"
            }
          },
          {
            "name": "allowedPrograms",
            "type": {
              "vec": "publicKey"
            }
          }
        ]
      }
    },
    {
      "name": "UpdateRulesetIx",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "collector",
            "type": "publicKey"
          },
          {
            "name": "checkSellerFeeBasisPoints",
            "type": "bool"
          },
          {
            "name": "disallowedAddresses",
            "type": {
              "vec": "publicKey"
            }
          },
          {
            "name": "allowedPrograms",
            "type": {
              "vec": "publicKey"
            }
          }
        ]
      }
    },
    {
      "name": "AccountBalance",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "address",
            "type": "publicKey"
          },
          {
            "name": "mint",
            "type": "publicKey"
          },
          {
            "name": "size",
            "type": "u64"
          },
          {
            "name": "balance",
            "type": "u64"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidMint",
      "msg": "Invalid mint"
    },
    {
      "code": 6001,
      "name": "InvalidCollector",
      "msg": "Invalid collector address"
    },
    {
      "code": 6002,
      "name": "InvalidAuthority",
      "msg": "Invalid authority address"
    },
    {
      "code": 6003,
      "name": "InvalidMintManager",
      "msg": "Invalid mint manager"
    },
    {
      "code": 6004,
      "name": "InvlaidHolderTokenAccount",
      "msg": "Invalid holder token account"
    },
    {
      "code": 6005,
      "name": "InvalidTargetTokenAccount",
      "msg": "Invalid target token account"
    },
    {
      "code": 6006,
      "name": "InvalidCloseTokenAccount",
      "msg": "Invalid token account to close"
    },
    {
      "code": 6007,
      "name": "InvalidHolderTokenAccount",
      "msg": "Invalid holder token account"
    },
    {
      "code": 6008,
      "name": "InvalidRuleset",
      "msg": "Invalid ruleset"
    },
    {
      "code": 6009,
      "name": "InvalidPreTransferInstruction",
      "msg": "Invalid pre transfer instruction"
    },
    {
      "code": 6010,
      "name": "InvalidPostTransferInstruction",
      "msg": "Invalid post transfer instruction"
    },
    {
      "code": 6011,
      "name": "ProgramDisallowed",
      "msg": "Disallowed program included in transfer"
    },
    {
      "code": 6012,
      "name": "ProgramNotAllowed",
      "msg": "Program not allowed in allowed programs to transfer"
    },
    {
      "code": 6013,
      "name": "UnknownAccount",
      "msg": "Unknown account found in instruction"
    },
    {
      "code": 6014,
      "name": "AccountNotFound",
      "msg": "Account not found in instruction"
    }
  ]
};
