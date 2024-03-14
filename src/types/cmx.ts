export type Cmx = {
  "version": "1.1.0",
  "name": "cmx",
  "instructions": [
    {
      "name": "mintNft",
      "accounts": [
        {
          "name": "config",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "candyMachine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mintReceiver",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "candyMachineWalletAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "launchStagesInfo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payFrom",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payTo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "notary",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "metadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "masterEdition",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "walletLimitInfo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "orderInfo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "slotHashes",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenMetadataProgram",
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
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "walletLimitBump",
          "type": "u8"
        },
        {
          "name": "inOrder",
          "type": "bool"
        },
        {
          "name": "userLimit",
          "type": {
            "option": "u8"
          }
        },
        {
          "name": "currTime",
          "type": "i64"
        }
      ]
    },
    {
      "name": "mintNftMip1",
      "accounts": [
        {
          "name": "config",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "candyMachine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mintReceiver",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "candyMachineWalletAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "launchStagesInfo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payFrom",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payTo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "notary",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "metadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "masterEdition",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenRecord",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "walletLimitInfo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "orderInfo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "slotHashes",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenMetadataProgram",
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
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "ruleSet",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "authorizationRulesProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "instructions",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "walletLimitBump",
          "type": "u8"
        },
        {
          "name": "inOrder",
          "type": "bool"
        },
        {
          "name": "userLimit",
          "type": {
            "option": "u8"
          }
        },
        {
          "name": "currTime",
          "type": "i64"
        }
      ]
    },
    {
      "name": "buyRaffleTicket",
      "accounts": [
        {
          "name": "config",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "candyMachine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "launchStagesInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "raffleTicket",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payFrom",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "raffleEscrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "paymentMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "walletLimitInfo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "slotHashes",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "notary",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
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
        }
      ],
      "args": [
        {
          "name": "walletLimitBump",
          "type": "u8"
        },
        {
          "name": "raffleTicketBump",
          "type": "u8"
        },
        {
          "name": "escrowBump",
          "type": "u8"
        },
        {
          "name": "currTime",
          "type": "i64"
        }
      ]
    },
    {
      "name": "checkRaffleTicket",
      "accounts": [
        {
          "name": "config",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "candyMachine",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "launchStagesInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "raffleTicket",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "currTime",
          "type": "i64"
        }
      ]
    },
    {
      "name": "settleRaffleTicket",
      "accounts": [
        {
          "name": "config",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "candyMachine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "launchStagesInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "raffleTicket",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "raffleEscrow",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "ATA where SPL tokens are held in escrow, owned by raffle_ticket"
          ]
        },
        {
          "name": "payTo",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "ATA owned by wallet authority that we pay to if we win"
          ]
        },
        {
          "name": "refundTo",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "or ATA owned by raffle_payer to refund SPL"
          ]
        },
        {
          "name": "tokenAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rafflePayer",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "orderInfo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "notary",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "metadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "updateAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "masterEdition",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "slotHashes",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
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
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "currTime",
          "type": "i64"
        }
      ]
    },
    {
      "name": "updateCandyMachine",
      "accounts": [
        {
          "name": "candyMachine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "launchStagesInfo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "notary",
          "type": {
            "option": "publicKey"
          }
        },
        {
          "name": "itemsAvailable",
          "type": {
            "option": "u64"
          }
        }
      ]
    },
    {
      "name": "initializeConfig",
      "accounts": [
        {
          "name": "config",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "authority",
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
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "args",
          "type": {
            "defined": "InitializeConfigArgs"
          }
        }
      ]
    },
    {
      "name": "initializeCandyMachine",
      "accounts": [
        {
          "name": "candyMachine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "launchStagesInfo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "orderInfo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "walletAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "config",
          "isMut": false,
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
          "name": "notary",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "args",
          "type": {
            "defined": "InitializeCandyMachineArgs"
          }
        }
      ]
    },
    {
      "name": "updateAuthority",
      "accounts": [
        {
          "name": "candyMachine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "launchStagesInfo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "newAuthority",
          "type": {
            "option": "publicKey"
          }
        }
      ]
    },
    {
      "name": "withdrawFunds",
      "accounts": [
        {
          "name": "config",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "updateConfig",
      "accounts": [
        {
          "name": "config",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": "UpdateConfigArgs"
          }
        }
      ]
    },
    {
      "name": "updateLaunchStages",
      "accounts": [
        {
          "name": "candyMachine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "launchStagesInfo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "stages",
          "type": {
            "vec": {
              "defined": "LaunchStageArgs"
            }
          }
        },
        {
          "name": "currTime",
          "type": "i64"
        },
        {
          "name": "notaryRequired",
          "type": {
            "vec": "bool"
          }
        }
      ]
    },
    {
      "name": "unverifyNonMasterEdition",
      "accounts": [
        {
          "name": "candyMachine",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "edition",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "metadata",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "populateOrder",
      "accounts": [
        {
          "name": "candyMachine",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "orderInfo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "size",
          "type": "u32"
        }
      ]
    },
    {
      "name": "withdrawOrderRent",
      "accounts": [
        {
          "name": "candyMachine",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "orderInfo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "freeze",
      "accounts": [
        {
          "name": "payer",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "freezeState",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "candyMachine",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "assetMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "assetToken",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "assetMasterEdition",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
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
      "name": "thaw",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "assetOwner",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "freezeState",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "candyMachine",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "assetMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "assetToken",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "assetMasterEdition",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
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
      "name": "updateFreezeState",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "freezeState",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "candyMachine",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "args",
          "type": {
            "defined": "UpdateFreezeStateArgs"
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "candyMachine",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "walletAuthority",
            "type": "publicKey"
          },
          {
            "name": "config",
            "type": "publicKey"
          },
          {
            "name": "itemsRedeemedNormal",
            "type": "u64"
          },
          {
            "name": "itemsRedeemedRaffle",
            "type": "u64"
          },
          {
            "name": "raffleTicketsPurchased",
            "type": "u64"
          },
          {
            "name": "uuid",
            "type": "string"
          },
          {
            "name": "itemsAvailable",
            "type": "u64"
          },
          {
            "name": "raffleSeed",
            "type": "u64"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "notary",
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "orderInfo",
            "type": "publicKey"
          },
          {
            "name": "isLite",
            "type": "bool"
          },
          {
            "name": "notaryRequired",
            "type": {
              "vec": "bool"
            }
          },
          {
            "name": "mip1Ruleset",
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "isOpenEdition",
            "type": {
              "option": "bool"
            }
          }
        ]
      }
    },
    {
      "name": "walletLimitInfo",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "redeemedNormal",
            "type": "u8"
          },
          {
            "name": "redeemedRaffleTickets",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "freezeState",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "expiry",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "walletLimitInfoPerStage",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "redeemed",
            "type": {
              "array": [
                {
                  "defined": "RedeemedDuringStage"
                },
                10
              ]
            }
          }
        ]
      }
    },
    {
      "name": "raffleTicket",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "ids",
            "type": {
              "vec": "u32"
            }
          },
          {
            "name": "candyMachine",
            "type": "publicKey"
          },
          {
            "name": "ticketBump",
            "type": "u8"
          },
          {
            "name": "escrowBump",
            "type": "u8"
          },
          {
            "name": "rafflePayer",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "launchStagesInfo",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "candyMachine",
            "type": "publicKey"
          },
          {
            "name": "stages",
            "type": {
              "vec": {
                "defined": "LaunchStage"
              }
            }
          }
        ]
      }
    },
    {
      "name": "config",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "gateway",
            "type": "string"
          },
          {
            "name": "cid",
            "type": "string"
          },
          {
            "name": "uuid",
            "type": "string"
          },
          {
            "name": "collectionName",
            "type": "string"
          },
          {
            "name": "symbol",
            "type": "string"
          },
          {
            "name": "sellerFeeBasisPoints",
            "docs": [
              "Royalty basis points that goes to creators in secondary sales (0-10000)"
            ],
            "type": "u16"
          },
          {
            "name": "creators",
            "type": {
              "vec": {
                "defined": "Creator"
              }
            }
          },
          {
            "name": "isMutable",
            "type": "bool"
          },
          {
            "name": "retainAuthority",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "order",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "filled",
            "type": "u32"
          },
          {
            "name": "candyMachine",
            "type": "publicKey"
          },
          {
            "name": "indices",
            "type": {
              "array": [
                "u32",
                50000
              ]
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "InitializeCandyMachineArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "cmBump",
            "type": "u8"
          },
          {
            "name": "launchStagesBump",
            "type": "u8"
          },
          {
            "name": "uuid",
            "type": "string"
          },
          {
            "name": "itemsAvailable",
            "type": "u64"
          },
          {
            "name": "stages",
            "type": {
              "vec": {
                "defined": "LaunchStageArgs"
              }
            }
          },
          {
            "name": "isLite",
            "type": "bool"
          },
          {
            "name": "notaryRequired",
            "type": {
              "vec": "bool"
            }
          },
          {
            "name": "mip1Ruleset",
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "isOpenEdition",
            "type": {
              "option": "bool"
            }
          }
        ]
      }
    },
    {
      "name": "InitializeConfigArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "gateway",
            "type": "string"
          },
          {
            "name": "cid",
            "type": "string"
          },
          {
            "name": "uuid",
            "type": "string"
          },
          {
            "name": "collectionName",
            "type": "string"
          },
          {
            "name": "symbol",
            "type": "string"
          },
          {
            "name": "sellerFeeBasisPoints",
            "type": "u16"
          },
          {
            "name": "creators",
            "type": {
              "vec": {
                "defined": "Creator"
              }
            }
          },
          {
            "name": "isMutable",
            "type": "bool"
          },
          {
            "name": "retainAuthority",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "UpdateConfigArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "gateway",
            "type": {
              "option": "string"
            }
          },
          {
            "name": "cid",
            "type": {
              "option": "string"
            }
          },
          {
            "name": "collectionName",
            "type": {
              "option": "string"
            }
          },
          {
            "name": "symbol",
            "type": {
              "option": "string"
            }
          },
          {
            "name": "sellerFeeBasisPoints",
            "type": {
              "option": "u16"
            }
          },
          {
            "name": "creators",
            "type": {
              "option": {
                "vec": {
                  "defined": "Creator"
                }
              }
            }
          }
        ]
      }
    },
    {
      "name": "UpdateFreezeStateArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "expiry",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "RedeemedDuringStage",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "redeemedNormal",
            "type": "u8"
          },
          {
            "name": "redeemedRaffleTickets",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "LaunchStage",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "stageType",
            "type": {
              "defined": "LaunchStageType"
            }
          },
          {
            "name": "startTime",
            "type": "i64"
          },
          {
            "name": "endTime",
            "type": "i64"
          },
          {
            "name": "walletLimit",
            "type": {
              "defined": "WalletLimitSpecification"
            }
          },
          {
            "name": "price",
            "type": "u64"
          },
          {
            "name": "stageSupply",
            "type": {
              "option": "u32"
            }
          },
          {
            "name": "previousStageUnmintedSupply",
            "type": "u32"
          },
          {
            "name": "mintedDuringStage",
            "type": "u32"
          },
          {
            "name": "paymentMint",
            "type": "publicKey"
          },
          {
            "name": "paymentAta",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "LaunchStageArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "stageType",
            "type": {
              "defined": "LaunchStageType"
            }
          },
          {
            "name": "startTime",
            "type": "i64"
          },
          {
            "name": "endTime",
            "type": "i64"
          },
          {
            "name": "walletLimit",
            "type": {
              "defined": "WalletLimitSpecification"
            }
          },
          {
            "name": "price",
            "type": "u64"
          },
          {
            "name": "stageSupply",
            "type": {
              "option": "u32"
            }
          },
          {
            "name": "paymentMintIndex",
            "type": "u8"
          },
          {
            "name": "paymentMintAtaBump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "Creator",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "address",
            "type": "publicKey"
          },
          {
            "name": "verified",
            "type": "bool"
          },
          {
            "name": "share",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "LaunchStageType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Invalid"
          },
          {
            "name": "NormalSale"
          },
          {
            "name": "Raffle"
          }
        ]
      }
    },
    {
      "name": "WalletLimitSpecification",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "NoLimit"
          },
          {
            "name": "FixedLimit",
            "fields": [
              {
                "name": "limit",
                "type": "u8"
              }
            ]
          },
          {
            "name": "VariableLimit"
          }
        ]
      }
    }
  ],
  "events": [
    {
      "name": "MintEvent",
      "fields": [
        {
          "name": "candyMachineId",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "itemsRedeemed",
          "type": "u64",
          "index": false
        }
      ]
    },
    {
      "name": "RaffleWinEvent",
      "fields": [
        {
          "name": "candyMachineId",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "itemsRedeemed",
          "type": "u64",
          "index": false
        },
        {
          "name": "winner",
          "type": "publicKey",
          "index": false
        }
      ]
    },
    {
      "name": "RaffleLossEvent",
      "fields": [
        {
          "name": "candyMachineId",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "itemsRedeemed",
          "type": "u64",
          "index": false
        }
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "IncorrectOwner",
      "msg": "Account does not have correct owner!"
    },
    {
      "code": 6001,
      "name": "Uninitialized",
      "msg": "Account is not initialized!"
    },
    {
      "code": 6002,
      "name": "MintMismatch",
      "msg": "Mint Mismatch!"
    },
    {
      "code": 6003,
      "name": "IndexGreaterThanLength",
      "msg": "Index greater than length!"
    },
    {
      "code": 6004,
      "name": "ConfigMustHaveAtleastOneEntry",
      "msg": "Config must have atleast one entry!"
    },
    {
      "code": 6005,
      "name": "NumericalOverflowError",
      "msg": "Numerical overflow error!"
    },
    {
      "code": 6006,
      "name": "TooManyCreators",
      "msg": "Can only provide up to 4 creators to candy machine!"
    },
    {
      "code": 6007,
      "name": "UuidMustBeExactly6Length",
      "msg": "Uuid must be exactly of 6 length"
    },
    {
      "code": 6008,
      "name": "NotEnoughTokens",
      "msg": "Not enough tokens to pay for this minting"
    },
    {
      "code": 6009,
      "name": "NotEnoughSOL",
      "msg": "Not enough SOL to pay for this minting"
    },
    {
      "code": 6010,
      "name": "TokenTransferFailed",
      "msg": "Token transfer failed"
    },
    {
      "code": 6011,
      "name": "CandyMachineEmpty",
      "msg": "Candy machine is empty!"
    },
    {
      "code": 6012,
      "name": "CandyMachineNotLiveYet",
      "msg": "Candy machine is not live yet!"
    },
    {
      "code": 6013,
      "name": "ConfigLineMismatch",
      "msg": "Number of config lines must be at least number of items available"
    },
    {
      "code": 6014,
      "name": "CIDLengthTooLong",
      "msg": "CID must be less than 255 characters"
    },
    {
      "code": 6015,
      "name": "CollectionNameTooLong",
      "msg": "Collection name must be less than 100 characters"
    },
    {
      "code": 6016,
      "name": "NotarySignatureNotProvided",
      "msg": "Notary signature not provided"
    },
    {
      "code": 6017,
      "name": "NotaryPublicKeyInvalid",
      "msg": "Invalid notary public key provided"
    },
    {
      "code": 6018,
      "name": "WalletLimitExceeded",
      "msg": "Wallet limit exceeded"
    },
    {
      "code": 6019,
      "name": "DerivedKeyInvalid",
      "msg": "Derived key invalid"
    },
    {
      "code": 6020,
      "name": "TooManyLaunchStages",
      "msg": "Too many launch stages (max 4)"
    },
    {
      "code": 6021,
      "name": "InvalidLaunchStagesInfoFields",
      "msg": "Invalid authority or candy_machine on launch_stages_info"
    },
    {
      "code": 6022,
      "name": "SymbolTooLong",
      "msg": "Symbol too long"
    },
    {
      "code": 6023,
      "name": "CurrentStageMismatch",
      "msg": "Current launch stage is invalid for stage required in the requested operation"
    },
    {
      "code": 6024,
      "name": "InvalidLaunchStage",
      "msg": "Invalid launch stage type"
    },
    {
      "code": 6025,
      "name": "TooManyRaffleStages",
      "msg": "Too many raffle related stages, only 1 is allowed"
    },
    {
      "code": 6026,
      "name": "LaunchStageStartAfterEnd",
      "msg": "Launch stage has start time after end time"
    },
    {
      "code": 6027,
      "name": "LaunchStagesOutOfTimeOrder",
      "msg": "Launch stage list is not in order of earliest stage to latest stage"
    },
    {
      "code": 6028,
      "name": "NoMatchingLaunchStage",
      "msg": "Cannot find matching stage for current time in launch stages"
    },
    {
      "code": 6029,
      "name": "TooManyRaffleTickets",
      "msg": "Allowing too many tickets to be purchased"
    },
    {
      "code": 6030,
      "name": "RaffleNotFound",
      "msg": "Raffle stage cannot be found"
    },
    {
      "code": 6031,
      "name": "RandomHashMismatch",
      "msg": "Random seed used to settle auction does not hash into expected value"
    },
    {
      "code": 6032,
      "name": "RaffleTicketEmpty",
      "msg": "There are no ids in the given raffle ticket"
    },
    {
      "code": 6033,
      "name": "RaffleTicketNotWinner",
      "msg": "Raffle ticket ID is not a winner"
    },
    {
      "code": 6034,
      "name": "NoLaunchStages",
      "msg": "Launch stages cannot be empty"
    },
    {
      "code": 6035,
      "name": "RaffleRequiresLimit",
      "msg": "Raffle stage type requires a wallet limit that is fixed"
    },
    {
      "code": 6036,
      "name": "StageNotActive",
      "msg": "No stage is active at this time"
    },
    {
      "code": 6037,
      "name": "TooFewItemsAvailable",
      "msg": "Trying to update to too few items"
    },
    {
      "code": 6038,
      "name": "AccountsAlreadyInUse",
      "msg": "Accounts assumed to be uninitialized are already in use"
    },
    {
      "code": 6039,
      "name": "AccountsUninitialized",
      "msg": "Accounts expected to be initialized are not initialized"
    },
    {
      "code": 6040,
      "name": "EditionKeyNotEdition",
      "msg": "Edition account's key indicates that it is not an edition"
    },
    {
      "code": 6041,
      "name": "OrderAccountNotPopulated",
      "msg": "Order account is not fully populated"
    },
    {
      "code": 6042,
      "name": "CannotMintInOrderAfterRandom",
      "msg": "Attempted to mint index that was already minted due to shuffle"
    },
    {
      "code": 6043,
      "name": "MintNotFinished",
      "msg": "Mint using this candy machine has not finished"
    },
    {
      "code": 6044,
      "name": "TokenOwnerMismatch",
      "msg": "Token account owner is not what we expected"
    },
    {
      "code": 6045,
      "name": "AuthorityHasToMintForSelf",
      "msg": "Authority has to mint for itself"
    },
    {
      "code": 6046,
      "name": "InvalidDiscriminator",
      "msg": "Discriminator different from what was expected"
    },
    {
      "code": 6047,
      "name": "UserLimitNeedsNotary",
      "msg": "User limit needs to have notary signature"
    },
    {
      "code": 6048,
      "name": "ReceivingTokenMismatch",
      "msg": "Given receiving token account(s) does not match expected"
    },
    {
      "code": 6049,
      "name": "RaffleDoesNotSupportSupply",
      "msg": "Raffle currently does not support supply per stage"
    },
    {
      "code": 6050,
      "name": "AtaMismatch",
      "msg": "Given ATA address is not what is expected"
    },
    {
      "code": 6051,
      "name": "InsufficientStageSupply",
      "msg": "Given stage supply will not allow the mint to complete"
    },
    {
      "code": 6052,
      "name": "CannotDeleteStages",
      "msg": "Cannot delete stages"
    },
    {
      "code": 6053,
      "name": "MissingUserLimit",
      "msg": "Missing user limit"
    },
    {
      "code": 6054,
      "name": "StageEmpty",
      "msg": "Stage supply has been exhausted"
    },
    {
      "code": 6055,
      "name": "InsufficientStageGap",
      "msg": "Stages do not have sufficient gap time between them"
    },
    {
      "code": 6056,
      "name": "GatewayTooLong",
      "msg": "Gateway too long"
    },
    {
      "code": 6057,
      "name": "SellerFeeBasisPointsOutOfRange",
      "msg": "Seller fee basis points must be between 0 and 10000"
    },
    {
      "code": 6058,
      "name": "FailedToDeserializeWalletLimitInfo",
      "msg": "Failed to deserialize wallet limit info"
    },
    {
      "code": 6059,
      "name": "CannotMintInLiteMode",
      "msg": "Cannot mint in lite mode after candy machine was used in normal mode"
    },
    {
      "code": 6060,
      "name": "CannotMintInNormalMode",
      "msg": "Cannot mint in normal mode after candy machine was used in lite mode"
    },
    {
      "code": 6061,
      "name": "NotaryRequiredLengthMismatch",
      "msg": "notary_required length must match stage length"
    },
    {
      "code": 6062,
      "name": "VariableLimitNotSupported",
      "msg": "Variable limit not supported"
    },
    {
      "code": 6063,
      "name": "NonLiteCandyMachineInvalidNotaryRequired",
      "msg": "Non lite candy machine must have notary_required set to all true"
    },
    {
      "code": 6064,
      "name": "CannotThawDueExpiry",
      "msg": "Cannot thaw due to expiry"
    },
    {
      "code": 6065,
      "name": "InvalidDelegate",
      "msg": "Invalid delegate"
    },
    {
      "code": 6066,
      "name": "CannotMintOpenEditions",
      "msg": "Cannot mint Open Editions"
    }
  ]
};

export const IDL: Cmx = {
  "version": "1.1.0",
  "name": "cmx",
  "instructions": [
    {
      "name": "mintNft",
      "accounts": [
        {
          "name": "config",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "candyMachine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mintReceiver",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "candyMachineWalletAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "launchStagesInfo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payFrom",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payTo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "notary",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "metadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "masterEdition",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "walletLimitInfo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "orderInfo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "slotHashes",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenMetadataProgram",
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
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "walletLimitBump",
          "type": "u8"
        },
        {
          "name": "inOrder",
          "type": "bool"
        },
        {
          "name": "userLimit",
          "type": {
            "option": "u8"
          }
        },
        {
          "name": "currTime",
          "type": "i64"
        }
      ]
    },
    {
      "name": "mintNftMip1",
      "accounts": [
        {
          "name": "config",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "candyMachine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mintReceiver",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "candyMachineWalletAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "launchStagesInfo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payFrom",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payTo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "notary",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "metadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "masterEdition",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenRecord",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "walletLimitInfo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "orderInfo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "slotHashes",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenMetadataProgram",
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
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "ruleSet",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "authorizationRulesProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "instructions",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "walletLimitBump",
          "type": "u8"
        },
        {
          "name": "inOrder",
          "type": "bool"
        },
        {
          "name": "userLimit",
          "type": {
            "option": "u8"
          }
        },
        {
          "name": "currTime",
          "type": "i64"
        }
      ]
    },
    {
      "name": "buyRaffleTicket",
      "accounts": [
        {
          "name": "config",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "candyMachine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "launchStagesInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "raffleTicket",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payFrom",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "raffleEscrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "paymentMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "walletLimitInfo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "slotHashes",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "notary",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
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
        }
      ],
      "args": [
        {
          "name": "walletLimitBump",
          "type": "u8"
        },
        {
          "name": "raffleTicketBump",
          "type": "u8"
        },
        {
          "name": "escrowBump",
          "type": "u8"
        },
        {
          "name": "currTime",
          "type": "i64"
        }
      ]
    },
    {
      "name": "checkRaffleTicket",
      "accounts": [
        {
          "name": "config",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "candyMachine",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "launchStagesInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "raffleTicket",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "currTime",
          "type": "i64"
        }
      ]
    },
    {
      "name": "settleRaffleTicket",
      "accounts": [
        {
          "name": "config",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "candyMachine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "launchStagesInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "raffleTicket",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "raffleEscrow",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "ATA where SPL tokens are held in escrow, owned by raffle_ticket"
          ]
        },
        {
          "name": "payTo",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "ATA owned by wallet authority that we pay to if we win"
          ]
        },
        {
          "name": "refundTo",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "or ATA owned by raffle_payer to refund SPL"
          ]
        },
        {
          "name": "tokenAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rafflePayer",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "orderInfo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "notary",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "metadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "updateAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "masterEdition",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "slotHashes",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
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
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "currTime",
          "type": "i64"
        }
      ]
    },
    {
      "name": "updateCandyMachine",
      "accounts": [
        {
          "name": "candyMachine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "launchStagesInfo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "notary",
          "type": {
            "option": "publicKey"
          }
        },
        {
          "name": "itemsAvailable",
          "type": {
            "option": "u64"
          }
        }
      ]
    },
    {
      "name": "initializeConfig",
      "accounts": [
        {
          "name": "config",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "authority",
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
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "args",
          "type": {
            "defined": "InitializeConfigArgs"
          }
        }
      ]
    },
    {
      "name": "initializeCandyMachine",
      "accounts": [
        {
          "name": "candyMachine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "launchStagesInfo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "orderInfo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "walletAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "config",
          "isMut": false,
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
          "name": "notary",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "args",
          "type": {
            "defined": "InitializeCandyMachineArgs"
          }
        }
      ]
    },
    {
      "name": "updateAuthority",
      "accounts": [
        {
          "name": "candyMachine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "launchStagesInfo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "newAuthority",
          "type": {
            "option": "publicKey"
          }
        }
      ]
    },
    {
      "name": "withdrawFunds",
      "accounts": [
        {
          "name": "config",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "updateConfig",
      "accounts": [
        {
          "name": "config",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": "UpdateConfigArgs"
          }
        }
      ]
    },
    {
      "name": "updateLaunchStages",
      "accounts": [
        {
          "name": "candyMachine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "launchStagesInfo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "stages",
          "type": {
            "vec": {
              "defined": "LaunchStageArgs"
            }
          }
        },
        {
          "name": "currTime",
          "type": "i64"
        },
        {
          "name": "notaryRequired",
          "type": {
            "vec": "bool"
          }
        }
      ]
    },
    {
      "name": "unverifyNonMasterEdition",
      "accounts": [
        {
          "name": "candyMachine",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "edition",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "metadata",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "populateOrder",
      "accounts": [
        {
          "name": "candyMachine",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "orderInfo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "size",
          "type": "u32"
        }
      ]
    },
    {
      "name": "withdrawOrderRent",
      "accounts": [
        {
          "name": "candyMachine",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "orderInfo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "freeze",
      "accounts": [
        {
          "name": "payer",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "freezeState",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "candyMachine",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "assetMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "assetToken",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "assetMasterEdition",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
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
      "name": "thaw",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "assetOwner",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "freezeState",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "candyMachine",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "assetMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "assetToken",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "assetMasterEdition",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
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
      "name": "updateFreezeState",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "freezeState",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "candyMachine",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "args",
          "type": {
            "defined": "UpdateFreezeStateArgs"
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "candyMachine",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "walletAuthority",
            "type": "publicKey"
          },
          {
            "name": "config",
            "type": "publicKey"
          },
          {
            "name": "itemsRedeemedNormal",
            "type": "u64"
          },
          {
            "name": "itemsRedeemedRaffle",
            "type": "u64"
          },
          {
            "name": "raffleTicketsPurchased",
            "type": "u64"
          },
          {
            "name": "uuid",
            "type": "string"
          },
          {
            "name": "itemsAvailable",
            "type": "u64"
          },
          {
            "name": "raffleSeed",
            "type": "u64"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "notary",
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "orderInfo",
            "type": "publicKey"
          },
          {
            "name": "isLite",
            "type": "bool"
          },
          {
            "name": "notaryRequired",
            "type": {
              "vec": "bool"
            }
          },
          {
            "name": "mip1Ruleset",
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "isOpenEdition",
            "type": {
              "option": "bool"
            }
          }
        ]
      }
    },
    {
      "name": "walletLimitInfo",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "redeemedNormal",
            "type": "u8"
          },
          {
            "name": "redeemedRaffleTickets",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "freezeState",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "expiry",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "walletLimitInfoPerStage",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "redeemed",
            "type": {
              "array": [
                {
                  "defined": "RedeemedDuringStage"
                },
                10
              ]
            }
          }
        ]
      }
    },
    {
      "name": "raffleTicket",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "ids",
            "type": {
              "vec": "u32"
            }
          },
          {
            "name": "candyMachine",
            "type": "publicKey"
          },
          {
            "name": "ticketBump",
            "type": "u8"
          },
          {
            "name": "escrowBump",
            "type": "u8"
          },
          {
            "name": "rafflePayer",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "launchStagesInfo",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "candyMachine",
            "type": "publicKey"
          },
          {
            "name": "stages",
            "type": {
              "vec": {
                "defined": "LaunchStage"
              }
            }
          }
        ]
      }
    },
    {
      "name": "config",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "gateway",
            "type": "string"
          },
          {
            "name": "cid",
            "type": "string"
          },
          {
            "name": "uuid",
            "type": "string"
          },
          {
            "name": "collectionName",
            "type": "string"
          },
          {
            "name": "symbol",
            "type": "string"
          },
          {
            "name": "sellerFeeBasisPoints",
            "docs": [
              "Royalty basis points that goes to creators in secondary sales (0-10000)"
            ],
            "type": "u16"
          },
          {
            "name": "creators",
            "type": {
              "vec": {
                "defined": "Creator"
              }
            }
          },
          {
            "name": "isMutable",
            "type": "bool"
          },
          {
            "name": "retainAuthority",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "order",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "filled",
            "type": "u32"
          },
          {
            "name": "candyMachine",
            "type": "publicKey"
          },
          {
            "name": "indices",
            "type": {
              "array": [
                "u32",
                50000
              ]
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "InitializeCandyMachineArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "cmBump",
            "type": "u8"
          },
          {
            "name": "launchStagesBump",
            "type": "u8"
          },
          {
            "name": "uuid",
            "type": "string"
          },
          {
            "name": "itemsAvailable",
            "type": "u64"
          },
          {
            "name": "stages",
            "type": {
              "vec": {
                "defined": "LaunchStageArgs"
              }
            }
          },
          {
            "name": "isLite",
            "type": "bool"
          },
          {
            "name": "notaryRequired",
            "type": {
              "vec": "bool"
            }
          },
          {
            "name": "mip1Ruleset",
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "isOpenEdition",
            "type": {
              "option": "bool"
            }
          }
        ]
      }
    },
    {
      "name": "InitializeConfigArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "gateway",
            "type": "string"
          },
          {
            "name": "cid",
            "type": "string"
          },
          {
            "name": "uuid",
            "type": "string"
          },
          {
            "name": "collectionName",
            "type": "string"
          },
          {
            "name": "symbol",
            "type": "string"
          },
          {
            "name": "sellerFeeBasisPoints",
            "type": "u16"
          },
          {
            "name": "creators",
            "type": {
              "vec": {
                "defined": "Creator"
              }
            }
          },
          {
            "name": "isMutable",
            "type": "bool"
          },
          {
            "name": "retainAuthority",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "UpdateConfigArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "gateway",
            "type": {
              "option": "string"
            }
          },
          {
            "name": "cid",
            "type": {
              "option": "string"
            }
          },
          {
            "name": "collectionName",
            "type": {
              "option": "string"
            }
          },
          {
            "name": "symbol",
            "type": {
              "option": "string"
            }
          },
          {
            "name": "sellerFeeBasisPoints",
            "type": {
              "option": "u16"
            }
          },
          {
            "name": "creators",
            "type": {
              "option": {
                "vec": {
                  "defined": "Creator"
                }
              }
            }
          }
        ]
      }
    },
    {
      "name": "UpdateFreezeStateArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "expiry",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "RedeemedDuringStage",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "redeemedNormal",
            "type": "u8"
          },
          {
            "name": "redeemedRaffleTickets",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "LaunchStage",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "stageType",
            "type": {
              "defined": "LaunchStageType"
            }
          },
          {
            "name": "startTime",
            "type": "i64"
          },
          {
            "name": "endTime",
            "type": "i64"
          },
          {
            "name": "walletLimit",
            "type": {
              "defined": "WalletLimitSpecification"
            }
          },
          {
            "name": "price",
            "type": "u64"
          },
          {
            "name": "stageSupply",
            "type": {
              "option": "u32"
            }
          },
          {
            "name": "previousStageUnmintedSupply",
            "type": "u32"
          },
          {
            "name": "mintedDuringStage",
            "type": "u32"
          },
          {
            "name": "paymentMint",
            "type": "publicKey"
          },
          {
            "name": "paymentAta",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "LaunchStageArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "stageType",
            "type": {
              "defined": "LaunchStageType"
            }
          },
          {
            "name": "startTime",
            "type": "i64"
          },
          {
            "name": "endTime",
            "type": "i64"
          },
          {
            "name": "walletLimit",
            "type": {
              "defined": "WalletLimitSpecification"
            }
          },
          {
            "name": "price",
            "type": "u64"
          },
          {
            "name": "stageSupply",
            "type": {
              "option": "u32"
            }
          },
          {
            "name": "paymentMintIndex",
            "type": "u8"
          },
          {
            "name": "paymentMintAtaBump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "Creator",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "address",
            "type": "publicKey"
          },
          {
            "name": "verified",
            "type": "bool"
          },
          {
            "name": "share",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "LaunchStageType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Invalid"
          },
          {
            "name": "NormalSale"
          },
          {
            "name": "Raffle"
          }
        ]
      }
    },
    {
      "name": "WalletLimitSpecification",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "NoLimit"
          },
          {
            "name": "FixedLimit",
            "fields": [
              {
                "name": "limit",
                "type": "u8"
              }
            ]
          },
          {
            "name": "VariableLimit"
          }
        ]
      }
    }
  ],
  "events": [
    {
      "name": "MintEvent",
      "fields": [
        {
          "name": "candyMachineId",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "itemsRedeemed",
          "type": "u64",
          "index": false
        }
      ]
    },
    {
      "name": "RaffleWinEvent",
      "fields": [
        {
          "name": "candyMachineId",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "itemsRedeemed",
          "type": "u64",
          "index": false
        },
        {
          "name": "winner",
          "type": "publicKey",
          "index": false
        }
      ]
    },
    {
      "name": "RaffleLossEvent",
      "fields": [
        {
          "name": "candyMachineId",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "itemsRedeemed",
          "type": "u64",
          "index": false
        }
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "IncorrectOwner",
      "msg": "Account does not have correct owner!"
    },
    {
      "code": 6001,
      "name": "Uninitialized",
      "msg": "Account is not initialized!"
    },
    {
      "code": 6002,
      "name": "MintMismatch",
      "msg": "Mint Mismatch!"
    },
    {
      "code": 6003,
      "name": "IndexGreaterThanLength",
      "msg": "Index greater than length!"
    },
    {
      "code": 6004,
      "name": "ConfigMustHaveAtleastOneEntry",
      "msg": "Config must have atleast one entry!"
    },
    {
      "code": 6005,
      "name": "NumericalOverflowError",
      "msg": "Numerical overflow error!"
    },
    {
      "code": 6006,
      "name": "TooManyCreators",
      "msg": "Can only provide up to 4 creators to candy machine!"
    },
    {
      "code": 6007,
      "name": "UuidMustBeExactly6Length",
      "msg": "Uuid must be exactly of 6 length"
    },
    {
      "code": 6008,
      "name": "NotEnoughTokens",
      "msg": "Not enough tokens to pay for this minting"
    },
    {
      "code": 6009,
      "name": "NotEnoughSOL",
      "msg": "Not enough SOL to pay for this minting"
    },
    {
      "code": 6010,
      "name": "TokenTransferFailed",
      "msg": "Token transfer failed"
    },
    {
      "code": 6011,
      "name": "CandyMachineEmpty",
      "msg": "Candy machine is empty!"
    },
    {
      "code": 6012,
      "name": "CandyMachineNotLiveYet",
      "msg": "Candy machine is not live yet!"
    },
    {
      "code": 6013,
      "name": "ConfigLineMismatch",
      "msg": "Number of config lines must be at least number of items available"
    },
    {
      "code": 6014,
      "name": "CIDLengthTooLong",
      "msg": "CID must be less than 255 characters"
    },
    {
      "code": 6015,
      "name": "CollectionNameTooLong",
      "msg": "Collection name must be less than 100 characters"
    },
    {
      "code": 6016,
      "name": "NotarySignatureNotProvided",
      "msg": "Notary signature not provided"
    },
    {
      "code": 6017,
      "name": "NotaryPublicKeyInvalid",
      "msg": "Invalid notary public key provided"
    },
    {
      "code": 6018,
      "name": "WalletLimitExceeded",
      "msg": "Wallet limit exceeded"
    },
    {
      "code": 6019,
      "name": "DerivedKeyInvalid",
      "msg": "Derived key invalid"
    },
    {
      "code": 6020,
      "name": "TooManyLaunchStages",
      "msg": "Too many launch stages (max 4)"
    },
    {
      "code": 6021,
      "name": "InvalidLaunchStagesInfoFields",
      "msg": "Invalid authority or candy_machine on launch_stages_info"
    },
    {
      "code": 6022,
      "name": "SymbolTooLong",
      "msg": "Symbol too long"
    },
    {
      "code": 6023,
      "name": "CurrentStageMismatch",
      "msg": "Current launch stage is invalid for stage required in the requested operation"
    },
    {
      "code": 6024,
      "name": "InvalidLaunchStage",
      "msg": "Invalid launch stage type"
    },
    {
      "code": 6025,
      "name": "TooManyRaffleStages",
      "msg": "Too many raffle related stages, only 1 is allowed"
    },
    {
      "code": 6026,
      "name": "LaunchStageStartAfterEnd",
      "msg": "Launch stage has start time after end time"
    },
    {
      "code": 6027,
      "name": "LaunchStagesOutOfTimeOrder",
      "msg": "Launch stage list is not in order of earliest stage to latest stage"
    },
    {
      "code": 6028,
      "name": "NoMatchingLaunchStage",
      "msg": "Cannot find matching stage for current time in launch stages"
    },
    {
      "code": 6029,
      "name": "TooManyRaffleTickets",
      "msg": "Allowing too many tickets to be purchased"
    },
    {
      "code": 6030,
      "name": "RaffleNotFound",
      "msg": "Raffle stage cannot be found"
    },
    {
      "code": 6031,
      "name": "RandomHashMismatch",
      "msg": "Random seed used to settle auction does not hash into expected value"
    },
    {
      "code": 6032,
      "name": "RaffleTicketEmpty",
      "msg": "There are no ids in the given raffle ticket"
    },
    {
      "code": 6033,
      "name": "RaffleTicketNotWinner",
      "msg": "Raffle ticket ID is not a winner"
    },
    {
      "code": 6034,
      "name": "NoLaunchStages",
      "msg": "Launch stages cannot be empty"
    },
    {
      "code": 6035,
      "name": "RaffleRequiresLimit",
      "msg": "Raffle stage type requires a wallet limit that is fixed"
    },
    {
      "code": 6036,
      "name": "StageNotActive",
      "msg": "No stage is active at this time"
    },
    {
      "code": 6037,
      "name": "TooFewItemsAvailable",
      "msg": "Trying to update to too few items"
    },
    {
      "code": 6038,
      "name": "AccountsAlreadyInUse",
      "msg": "Accounts assumed to be uninitialized are already in use"
    },
    {
      "code": 6039,
      "name": "AccountsUninitialized",
      "msg": "Accounts expected to be initialized are not initialized"
    },
    {
      "code": 6040,
      "name": "EditionKeyNotEdition",
      "msg": "Edition account's key indicates that it is not an edition"
    },
    {
      "code": 6041,
      "name": "OrderAccountNotPopulated",
      "msg": "Order account is not fully populated"
    },
    {
      "code": 6042,
      "name": "CannotMintInOrderAfterRandom",
      "msg": "Attempted to mint index that was already minted due to shuffle"
    },
    {
      "code": 6043,
      "name": "MintNotFinished",
      "msg": "Mint using this candy machine has not finished"
    },
    {
      "code": 6044,
      "name": "TokenOwnerMismatch",
      "msg": "Token account owner is not what we expected"
    },
    {
      "code": 6045,
      "name": "AuthorityHasToMintForSelf",
      "msg": "Authority has to mint for itself"
    },
    {
      "code": 6046,
      "name": "InvalidDiscriminator",
      "msg": "Discriminator different from what was expected"
    },
    {
      "code": 6047,
      "name": "UserLimitNeedsNotary",
      "msg": "User limit needs to have notary signature"
    },
    {
      "code": 6048,
      "name": "ReceivingTokenMismatch",
      "msg": "Given receiving token account(s) does not match expected"
    },
    {
      "code": 6049,
      "name": "RaffleDoesNotSupportSupply",
      "msg": "Raffle currently does not support supply per stage"
    },
    {
      "code": 6050,
      "name": "AtaMismatch",
      "msg": "Given ATA address is not what is expected"
    },
    {
      "code": 6051,
      "name": "InsufficientStageSupply",
      "msg": "Given stage supply will not allow the mint to complete"
    },
    {
      "code": 6052,
      "name": "CannotDeleteStages",
      "msg": "Cannot delete stages"
    },
    {
      "code": 6053,
      "name": "MissingUserLimit",
      "msg": "Missing user limit"
    },
    {
      "code": 6054,
      "name": "StageEmpty",
      "msg": "Stage supply has been exhausted"
    },
    {
      "code": 6055,
      "name": "InsufficientStageGap",
      "msg": "Stages do not have sufficient gap time between them"
    },
    {
      "code": 6056,
      "name": "GatewayTooLong",
      "msg": "Gateway too long"
    },
    {
      "code": 6057,
      "name": "SellerFeeBasisPointsOutOfRange",
      "msg": "Seller fee basis points must be between 0 and 10000"
    },
    {
      "code": 6058,
      "name": "FailedToDeserializeWalletLimitInfo",
      "msg": "Failed to deserialize wallet limit info"
    },
    {
      "code": 6059,
      "name": "CannotMintInLiteMode",
      "msg": "Cannot mint in lite mode after candy machine was used in normal mode"
    },
    {
      "code": 6060,
      "name": "CannotMintInNormalMode",
      "msg": "Cannot mint in normal mode after candy machine was used in lite mode"
    },
    {
      "code": 6061,
      "name": "NotaryRequiredLengthMismatch",
      "msg": "notary_required length must match stage length"
    },
    {
      "code": 6062,
      "name": "VariableLimitNotSupported",
      "msg": "Variable limit not supported"
    },
    {
      "code": 6063,
      "name": "NonLiteCandyMachineInvalidNotaryRequired",
      "msg": "Non lite candy machine must have notary_required set to all true"
    },
    {
      "code": 6064,
      "name": "CannotThawDueExpiry",
      "msg": "Cannot thaw due to expiry"
    },
    {
      "code": 6065,
      "name": "InvalidDelegate",
      "msg": "Invalid delegate"
    },
    {
      "code": 6066,
      "name": "CannotMintOpenEditions",
      "msg": "Cannot mint Open Editions"
    }
  ]
};
