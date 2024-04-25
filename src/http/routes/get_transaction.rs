use serde_json::{json, Value};
use data_manager::prelude::TransactionManager;
use crate::global::data_manager::GLOBAL_DATA;
use crate::http::interfaces::get_transaction::GetTransactionRequest;
use crate::http::interfaces::params::optional::OptionalParams;


pub async fn get_transaction(request: &GetTransactionRequest, _params: OptionalParams) -> Value {
    let manager = GLOBAL_DATA.read().expect("Failed to read Global Data");
    let data = manager.get_transaction(&request.0.0);
    match data {
        None => {
            Value::Null
        },
        Some(data) => {
            json!({
                "blockTime": data.block_time,
                "meta": {
                    "computeUnitsConsumed": data.compute_units_consumed,
                    "err": null,
                    "fee": data.fee,
                    "innerInstructions": data.inner_instruction.iter()
                    .map(|instr| json!({
                        "index": instr.index,
                        "instructions": [
                            {
                                "accounts": instr.accounts,
                                "data": instr.encode(),
                                "programIdIndex": instr.program,
                                "stackHeight": instr.stack_height
                            }
                        ]
                    })).collect::<Vec<Value>>(),
                    "loadedAddresses": { //TODO
                        "readonly": [],
                        "writable": []
                    },
                    "logMessages": data.log_messages,
                    "postBalances": data.post_balances,
                    "preBalances": data.pre_balances,
                    "postTokenBalances": [],
                    "preTokenBalances": [],
                    "rewards": [],
                    "status": {
                        "Ok": null
                    }
                },
                "slot": data.slot,
                "transaction": {
                    "recentBlockhash": data.recent_blockhash,
                    "message": {
                        "accountKeys": data.accounts.iter()
                                .map(|key| key.pubkey.to_string())
                                .collect::<Vec<String>>(),
                        "header": {
                            "numReadonlySignedAccounts": data.accounts.iter()
                            .filter(|meta| meta.signer && !meta.writable)
                            .count(),
                            "numReadonlyUnsignedAccounts": data.accounts.iter()
                            .filter(|meta| !meta.signer && !meta.writable)
                            .count(),
                            "numRequiredSignatures": data.accounts.iter()
                            .filter(|meta| meta.signer)
                            .count()
                        },
                    },
                    "instructions": data.instructions.iter()
                        .map(|instr| json!({
                            "accounts": instr.accounts,
                            "data": instr.encode(),
                            "programIdIndex": instr.program,
                            "stackHeight": null
                        })).collect::<Vec<Value>>(),
                },
                "signatures": data.signatures
            })
        }
    }
}

/*
"innerInstructions": [
                {
                    "index": 2,
                    "instructions": [
                        {
                            "accounts": [
                                6,
                                10,
                                12
                            ],
                            "data": "3px3hhrW2tYw",
                            "programIdIndex": 16,
                            "stackHeight": 2
                        }
                    ]
                }
            ],
 */

// {
//     "accounts": [
//         0,
//         5,
//         14
//     ],
//     "data": "6mJFQCt94hG4CKNYKgVcwnxoiXP4PmMCqB4Hz9BDvni4TZWFxQiZhy",
//     "programIdIndex": 15,
//     "stackHeight": null
// },


/*
{
    "jsonrpc": "2.0",
    "result": {
        "blockTime": 1713479970,
        "meta": {
            "computeUnitsConsumed": 54487,
            "err": null,
            "fee": 6920,
            "innerInstructions": [],
            "loadedAddresses": {
                "readonly": [],
                "writable": []
            },
            "logMessages": [
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s invoke [1]",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s consumed 4528 of 480000 compute units",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s success",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s invoke [1]",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s consumed 4504 of 475472 compute units",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s success",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s invoke [1]",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s consumed 4450 of 470968 compute units",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s success",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s invoke [1]",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s consumed 4447 of 466518 compute units",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s success",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s invoke [1]",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s consumed 4574 of 462071 compute units",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s success",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s invoke [1]",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s consumed 4509 of 457497 compute units",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s success",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s invoke [1]",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s consumed 4308 of 452988 compute units",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s success",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s invoke [1]",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s consumed 4523 of 448680 compute units",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s success",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s invoke [1]",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s consumed 4766 of 444157 compute units",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s success",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s invoke [1]",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s consumed 4542 of 439391 compute units",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s success",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s invoke [1]",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s consumed 4495 of 434849 compute units",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s success",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s invoke [1]",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s consumed 4541 of 430354 compute units",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s success",
                "Program ComputeBudget111111111111111111111111111111 invoke [1]",
                "Program ComputeBudget111111111111111111111111111111 success",
                "Program ComputeBudget111111111111111111111111111111 invoke [1]",
                "Program ComputeBudget111111111111111111111111111111 success"
            ],
            "postBalances": [
                472114816462,
                23942400,
                23942400,
                23942400,
                23942400,
                23942400,
                23942400,
                23942400,
                23942400,
                23942400,
                23942400,
                23942400,
                23942400,
                1,
                1169280,
                1141440
            ],
            "postTokenBalances": [],
            "preBalances": [
                472114823382,
                23942400,
                23942400,
                23942400,
                23942400,
                23942400,
                23942400,
                23942400,
                23942400,
                23942400,
                23942400,
                23942400,
                23942400,
                1,
                1169280,
                1141440
            ],
            "preTokenBalances": [],
            "rewards": [],
            "status": {
                "Ok": null
            }
        },
        "slot": 293327967,
        "transaction": {
            "message": {
                "accountKeys": [
                    "5U3bH5b6XtG99aVWLqwVzYPVpQiFHytBD68Rz2eFPZd7",
                    "48AUy2tdzzTpaPrXYTjNp7XNPRTyRgGoMsy9Gp4cfeiw",
                    "4Eid3xrND87TJ4Gv8ZyDqByzj3C88YFT5xh1MhhcPFwE",
                    "7gkPonHMUN4XjZBG3JobN13MmkKMwt6cEq2udq4AbmkB",
                    "87tBQbpwcKFNJSteADZ99L88podsPLgaenRfUsYL13vH",
                    "8UnMavdLbiSvktd36PvAMrk3NUs9rzbv7HqJrPZbKMA8",
                    "8Y4jhVcQvQZWjMarM855NMkVuua78FS8Uwy58TjcnUWs",
                    "ASqqyfu1t3kqzkwwR8fnJ8nzhChScojL3ChAgrv7g4Hf",
                    "BJYbzMFBUdUTaQRC4em3pKenSe1ttd9VjDKkSAxRTAwh",
                    "D4MDtMcBT44Ef5poHZA6KBVVfnE2mkzvBZu64j27RXme",
                    "DMyZyzfNR1uhqYcFGrNftirkveCbyNVLnCB8HgShYz4V",
                    "DuG45Td6dgJBe64Ebymb1WjBys16L1VTQdoAURdsviqN",
                    "Grp9rsTTCP9shKfbRQ78N4YxZMTGuvpcrjEDicRudbF6",
                    "ComputeBudget111111111111111111111111111111",
                    "SysvarC1ock11111111111111111111111111111111",
                    "gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s"
                ],
                "header": {
                    "numReadonlySignedAccounts": 0,
                    "numReadonlyUnsignedAccounts": 3,
                    "numRequiredSignatures": 1
                },
                "instructions": [
                    {
                        "accounts": [
                            0,
                            5,
                            14
                        ],
                        "data": "6mJFQCt94hG4CKNYKgVcwnxoiXP4PmMCqB4Hz9BDvni4TZWFxQiZhy",
                        "programIdIndex": 15,
                        "stackHeight": null
                    },
                    {
                        "accounts": [
                            0,
                            4,
                            14
                        ],
                        "data": "6mJFQCt94hG4CKNYKgVcweHGccw1hMj6RZ9NkCx4i6AwMo6ykzQDSX",
                        "programIdIndex": 15,
                        "stackHeight": null
                    },
                    {
                        "accounts": [
                            0,
                            11,
                            14
                        ],
                        "data": "6mJFQCt94hG4CKNYKgVcwoiWEgK2nd1PazN1NggW2aASnziUroYHd1",
                        "programIdIndex": 15,
                        "stackHeight": null
                    },
                    {
                        "accounts": [
                            0,
                            8,
                            14
                        ],
                        "data": "6mJFQCt94hG4CKNYKgVcweXe5bzue3WgTfnntHKza6XbezuaMRVBE7",
                        "programIdIndex": 15,
                        "stackHeight": null
                    },
                    {
                        "accounts": [
                            0,
                            1,
                            14
                        ],
                        "data": "6mJFQCt94hG4CKNYKgVcwzw34c4WJxuWeq94RFmfa3YundAHYDWPyH",
                        "programIdIndex": 15,
                        "stackHeight": null
                    },
                    {
                        "accounts": [
                            0,
                            3,
                            14
                        ],
                        "data": "6mJFQCt94hG4CKNYKgVcwmaxbdmyC4SEaWi8LTB2XxD4ppBGo5Xksq",
                        "programIdIndex": 15,
                        "stackHeight": null
                    },
                    {
                        "accounts": [
                            0,
                            10,
                            14
                        ],
                        "data": "6mJFQCt94hG4CKNYKgVcwhZraWBva2fHYQyHRArq861m6Rpv1YHnVd",
                        "programIdIndex": 15,
                        "stackHeight": null
                    },
                    {
                        "accounts": [
                            0,
                            9,
                            14
                        ],
                        "data": "6mJFQCt94hG4CKNYKgVcwx8gKorZmyq6cLnYL7V5ejvKDS4y1hgqKM",
                        "programIdIndex": 15,
                        "stackHeight": null
                    },
                    {
                        "accounts": [
                            0,
                            12,
                            14
                        ],
                        "data": "6mJFQCt94hG4CKNYKgVcwmCKbDNmkGGCarPdusU9QrnhHtU8oSZxy5",
                        "programIdIndex": 15,
                        "stackHeight": null
                    },
                    {
                        "accounts": [
                            0,
                            2,
                            14
                        ],
                        "data": "6mJFQCt94hG4CKNYKgVcwnQwjJEtK7dBSQaDjTWwVwsY53UJunFbXm",
                        "programIdIndex": 15,
                        "stackHeight": null
                    },
                    {
                        "accounts": [
                            0,
                            7,
                            14
                        ],
                        "data": "6mJFQCt94hG4CKNYKgVcwxQ9TLaDTY3qm5FkdVo8mPYTTNRS13yARm",
                        "programIdIndex": 15,
                        "stackHeight": null
                    },
                    {
                        "accounts": [
                            0,
                            6,
                            14
                        ],
                        "data": "6mJFQCt94hG4CKNYKgVcwoNr5zRCD4QzsLHczKV1nAjJ8GKbusLbKD",
                        "programIdIndex": 15,
                        "stackHeight": null
                    },
                    {
                        "accounts": [],
                        "data": "E6YYnj",
                        "programIdIndex": 13,
                        "stackHeight": null
                    },
                    {
                        "accounts": [],
                        "data": "3gELC8A682aT",
                        "programIdIndex": 13,
                        "stackHeight": null
                    }
                ],
                "recentBlockhash": "G2bzUgG6Y5kbgEhU6s6Lxh4PErK3febtUW8Cf6PRgaEu"
            },
            "signatures": [
                "534z3thLmap6KcW2TLR69FNbDD2Yxz4kNQAkpeVGnNYYjNLwzeskcemFUceCUQZDjbuaEHfiBzfSyzpTQDUBrEGc"
            ]
        }
    },
    "id": 1
}
 */






/*
{
    "jsonrpc": "2.0",
    "result": {
        "blockTime": 1713479970,
        "meta": {
            "computeUnitsConsumed": 54487,
            "err": null,
            "fee": 6920,
            "innerInstructions": [],
            "loadedAddresses": {
                "readonly": [],
                "writable": []
            },
            "logMessages": [
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s invoke [1]",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s consumed 4528 of 480000 compute units",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s success",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s invoke [1]",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s consumed 4504 of 475472 compute units",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s success",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s invoke [1]",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s consumed 4450 of 470968 compute units",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s success",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s invoke [1]",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s consumed 4447 of 466518 compute units",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s success",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s invoke [1]",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s consumed 4574 of 462071 compute units",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s success",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s invoke [1]",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s consumed 4509 of 457497 compute units",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s success",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s invoke [1]",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s consumed 4308 of 452988 compute units",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s success",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s invoke [1]",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s consumed 4523 of 448680 compute units",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s success",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s invoke [1]",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s consumed 4766 of 444157 compute units",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s success",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s invoke [1]",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s consumed 4542 of 439391 compute units",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s success",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s invoke [1]",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s consumed 4495 of 434849 compute units",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s success",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s invoke [1]",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s consumed 4541 of 430354 compute units",
                "Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s success",
                "Program ComputeBudget111111111111111111111111111111 invoke [1]",
                "Program ComputeBudget111111111111111111111111111111 success",
                "Program ComputeBudget111111111111111111111111111111 invoke [1]",
                "Program ComputeBudget111111111111111111111111111111 success"
            ],
            "postBalances": [
                472114816462,
                23942400,
                23942400,
                23942400,
                23942400,
                23942400,
                23942400,
                23942400,
                23942400,
                23942400,
                23942400,
                23942400,
                23942400,
                1,
                1169280,
                1141440
            ],
            "postTokenBalances": [],
            "preBalances": [
                472114823382,
                23942400,
                23942400,
                23942400,
                23942400,
                23942400,
                23942400,
                23942400,
                23942400,
                23942400,
                23942400,
                23942400,
                23942400,
                1,
                1169280,
                1141440
            ],
            "preTokenBalances": [],
            "rewards": [],
            "status": {
                "Ok": null
            }
        },
        "slot": 293327967,
        "transaction": [
            "AcnaGTe0ramfEd+NPHEIci7soNo4h3m5WgzbKaPzFeiaPZU6rOSF/NSE1VbaauU2d5K4ENwuzcdsDr2PWMlEqwUBAAMQQlxb88UapZ0T6mWzABhtX/lDiPrAaUMbsl4vmXpBgd4uaNkUmtY9QvTjvF0NYEkOMjZ2d/oYLU7uFFLxpY0+9DAWrEbk7/nRdvxVCkaxPtQCgHp08MVoGEfuRXzlQCXhY1TWhcJH8e/Db5Z2F77/ouvVqV2P/hfeRJhQXJIwZ7hpxSl/qWelE3LVYXT89yJbISY1Wb+9tc8D7/SvbCIS6m8fxfZ6LmNwYCpOrGHovSx4DC/Z/xfRv+8c29dmAUzRb/cJvx8P20bZka5oORY98eBQ0UNCnXN/xlC2shQ0eAiMV+5fLlj2GW6EU0Qplhj/V1Kk3tDKCuGJ8r9/7ulAipkTehg1Tvp/toQIidBZ/bBMRqbOIb6Xq2DZrZPpGsdYsyfZzw7NeToXX6cKyNLcEJ1EYnWOVWlixKh7AuxPPxW3q9Jadt2v/fhHIk8DGYzLknI/kLJCnPM/Duy5bjUqhr+vdznLb+PhxXoKwI4dkx6eYGLUdvpXgE4WWrVytbYh66IUTwSzr1k4LZK4y8MXAAi0opRaAfNugfkt/euMxRkDBkZv5SEXMv/srbpyw5vnvIzlu8X3EmssQ5s6QAAAAAan1RcYx3TJKFZjmGkdXraLXrijm0ttXHNVWyEAAAAAChqYM6N2VStWt8oN7RkpFwBX6Cegxif0tke57pCZr7TfSHL6u1YuQ4lEbwXmteAHcoKJKzOi16028bE7PitxYg4PAwAFDigCAAAADQAAAAEAAAAAAAAAfAlJFAAAAAAW9xkAAAAAAF3UexEAAAAADwMABA4oAgAAAA0AAAABAAAAAAAAABzOGB4AAAAA+RQDAAAAAABd1HsRAAAAAA8DAAsOKAIAAAANAAAAAQAAAAAAAACETP8CAAAAABMtAAAAAAAAXdR7EQAAAAAPAwAIDigCAAAADQAAAAEAAAAAAAAAH4XWAQAAAAAQJwAAAAAAAF3UexEAAAAADwMAAQ4oAgAAAA0AAAABAAAAAAAAAP9VGpMBAAAAr1IlAAAAAABd1HsRAAAAAA8DAAMOKAIAAAANAAAAAQAAAAAAAABs72wUAAAAAIw2AQAAAAAAXdR7EQAAAAAPAwAKDigCAAAADQAAAAEAAAAAAAAAQNl6b0cAAAA/LEIGAAAAAF3UexEAAAAADwMACQ4oAgAAAA0AAAABAAAAAAAAAOCgaCoAAAAA4JMEAAAAAABd1HsRAAAAAA8DAAwOKAIAAAANAAAAAQAAAAAAAABop2EBAAAAANMZAAAAAAAAXdR7EQAAAAAPAwACDigCAAAADQAAAAEAAAAAAAAAdgKp9AAAAADDBBoAAAAAAF3UexEAAAAADwMABw4oAgAAAA0AAAABAAAAAAAAAOONSwwAAAAApTsBAAAAAABd1HsRAAAAAA8DAAYOKAIAAAANAAAAAQAAAAAAAACAlT88AAAAAAA1DAAAAAAAXdR7EQAAAAANAAUCAFMHAA0ACQOgDwAAAAAAAA==",
            "base64"
        ]
    },
    "id": 1
}
 */

/*
None => {
            Value::Null
        }
        Some(meta) => {
            let data = manager.get_account_data(&request.0[0]);
            let bytes = match data {
                None => {
                    json!([
                        "",
                        "base64"
                    ])
                }
                Some(data) => {
                    space = data.bytes.len();
                    json!([
                        data.to_base64(),
                        "base64"
                    ])
                }
            };
            json!({
                "data": bytes,
                "executable": meta.executable,
                "lamports": meta.lamports,
                "owner": meta.owner.to_string(),
                "rentEpoch": 0,
                "space": space
            })
        }
 */