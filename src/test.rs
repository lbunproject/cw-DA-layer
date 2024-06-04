#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{from_binary, Addr, StdResult};
    use crate::error::ContractError;
    use crate::msg::{MigrateMsg, InstantiateMsg, ExecuteMsg, QueryMsg, ContentsMsg, Blob, BlobInfoResponse, StateInfoResponse};
    use crate::state::{CONTRACT_STATE, ContractState, BLOB_DATA, BlobData};
    use crate::execute::{try_change_manager, try_submit_blob};
    use crate::query::{get_state_info, get_blob_by_id, get_blob_by_block};
    use crate::helper::check_authorized_or_manager;
    use crate::contract::{instantiate, execute, query, AUTHORIZED_ADDRESS};

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {};
        let info = mock_info(AUTHORIZED_ADDRESS, &[]);

        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(res.attributes[0].value, "instantiate");

        let state = CONTRACT_STATE.load(&deps.storage).unwrap();
        assert_eq!(state.manager, Addr::unchecked(AUTHORIZED_ADDRESS));
        assert_eq!(state.blob_count, 0);
    }

    #[test]
    fn test_submit_blob() {
        let mut deps = mock_dependencies();

        let instantiate_msg = InstantiateMsg {};
        let info = mock_info(AUTHORIZED_ADDRESS, &[]);
        instantiate(deps.as_mut(), mock_env(), info.clone(), instantiate_msg).unwrap();

        let submit_msg = ExecuteMsg::SubmitBlob {
            contents: ContentsMsg {
                action: "submit".to_string(),
                blob_id: 1,
                message: vec!["data1".to_string()], // Changed to Vec<String>
            },
        };

        let res = execute(deps.as_mut(), mock_env(), info, submit_msg).unwrap();
        assert_eq!(res.attributes[0].value, "submit_blob");

        let state = CONTRACT_STATE.load(&deps.storage).unwrap();
        assert_eq!(state.blob_count, 1);

        let blob_data = BLOB_DATA.load(&deps.storage).unwrap();
        assert_eq!(blob_data.blobs.len(), 1);
        assert_eq!(blob_data.blobs[0].blob_id, 1);
        assert_eq!(blob_data.blobs[0].data, vec!["data1".to_string()]);
    }

    #[test]
    fn test_get_blob_by_id() {
        let mut deps = mock_dependencies();

        let instantiate_msg = InstantiateMsg {};
        let info = mock_info(AUTHORIZED_ADDRESS, &[]);
        instantiate(deps.as_mut(), mock_env(), info.clone(), instantiate_msg).unwrap();

        let submit_msg = ExecuteMsg::SubmitBlob {
            contents: ContentsMsg {
                action: "submit".to_string(),
                blob_id: 1,
                message: vec!["data1".to_string()], // Changed to Vec<String>
            },
        };

        execute(deps.as_mut(), mock_env(), info.clone(), submit_msg).unwrap();

        let query_msg = QueryMsg::GetBlobById { blob_id: 1 };

        let res = query(deps.as_ref(), mock_env(), query_msg).unwrap();
        let blob: BlobInfoResponse = from_binary(&res).unwrap();
        assert_eq!(blob.blob_id, 1);
        assert_eq!(blob.data, vec!["data1".to_string()]);
    }

    #[test]
    fn test_get_blob_by_block() {
        let mut deps = mock_dependencies();

        let instantiate_msg = InstantiateMsg {};
        let info = mock_info(AUTHORIZED_ADDRESS, &[]);
        instantiate(deps.as_mut(), mock_env(), info.clone(), instantiate_msg).unwrap();

        let submit_msg = ExecuteMsg::SubmitBlob {
            contents: ContentsMsg {
                action: "submit".to_string(),
                blob_id: 1,
                message: vec!["data1".to_string()], // Changed to Vec<String>
            },
        };

        execute(deps.as_mut(), mock_env(), info.clone(), submit_msg).unwrap();

        let block_number = mock_env().block.height;

        let query_msg = QueryMsg::GetBlobByBlock { terra_block_number: block_number };

        let res = query(deps.as_ref(), mock_env(), query_msg).unwrap();
        let blob: BlobInfoResponse = from_binary(&res).unwrap();
        assert_eq!(blob.blob_id, 1);
        assert_eq!(blob.terra_block_number, block_number);
        assert_eq!(blob.data, vec!["data1".to_string()]);
    }

    #[test]
    fn test_get_state_info() {
        let mut deps = mock_dependencies();

        let instantiate_msg = InstantiateMsg {};
        let info = mock_info(AUTHORIZED_ADDRESS, &[]);
        instantiate(deps.as_mut(), mock_env(), info.clone(), instantiate_msg).unwrap();

        let query_msg = QueryMsg::GetStateInfo {};

        let res = query(deps.as_ref(), mock_env(), query_msg).unwrap();
        let state_info: StateInfoResponse = from_binary(&res).unwrap();
        assert_eq!(state_info.manager, Addr::unchecked(AUTHORIZED_ADDRESS));
        assert_eq!(state_info.blob_count, 0);
    }
}
