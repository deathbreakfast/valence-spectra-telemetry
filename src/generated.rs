pub mod helpers {
    #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
    pub struct ValenceDeletionLog {
        pub outcome: String,
        pub root_table: String,
        pub root_record_id: String,
        pub node_count: String,
        pub max_depth: String,
        pub restrict_count: String,
        pub trait_expansion_count: String,
    }
    pub struct ValenceDeletionLogLogger;
    impl ValenceDeletionLogLogger {
        #[doc = r" Emit with an explicit timestamp (preserved through buffered drain)."]
        pub fn log_at(
            outcome: String,
            root_table: String,
            root_record_id: String,
            node_count: String,
            max_depth: String,
            restrict_count: String,
            trait_expansion_count: String,
            _ts: chrono::DateTime<chrono::Utc>,
        ) {
            let mut map = serde_json::Map::new();
            map.insert("outcome".to_string(), serde_json::json!(outcome));
            map.insert("root_table".to_string(), serde_json::json!(root_table));
            map.insert(
                "root_record_id".to_string(),
                serde_json::json!(root_record_id),
            );
            map.insert("node_count".to_string(), serde_json::json!(node_count));
            map.insert("max_depth".to_string(), serde_json::json!(max_depth));
            map.insert(
                "restrict_count".to_string(),
                serde_json::json!(restrict_count),
            );
            map.insert(
                "trait_expansion_count".to_string(),
                serde_json::json!(trait_expansion_count),
            );
            let fields = serde_json::Value::Object(map);
            ::spectra_core::try_log_event_now("valence_deletion_log", &fields);
        }
        pub fn log(
            outcome: String,
            root_table: String,
            root_record_id: String,
            node_count: String,
            max_depth: String,
            restrict_count: String,
            trait_expansion_count: String,
        ) {
            Self::log_at(
                outcome,
                root_table,
                root_record_id,
                node_count,
                max_depth,
                restrict_count,
                trait_expansion_count,
                chrono::Utc::now(),
            );
        }
    }
    #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
    pub struct ValenceErrorLog {
        pub source: String,
        pub operation: String,
        pub table: String,
        pub database_type: String,
        pub message: String,
    }
    pub struct ValenceErrorLogLogger;
    impl ValenceErrorLogLogger {
        #[doc = r" Emit with an explicit timestamp (preserved through buffered drain)."]
        pub fn log_at(
            source: String,
            operation: String,
            table: String,
            database_type: String,
            message: String,
            _ts: chrono::DateTime<chrono::Utc>,
        ) {
            let mut map = serde_json::Map::new();
            map.insert("source".to_string(), serde_json::json!(source));
            map.insert("operation".to_string(), serde_json::json!(operation));
            map.insert("table".to_string(), serde_json::json!(table));
            map.insert(
                "database_type".to_string(),
                serde_json::json!(database_type),
            );
            map.insert("message".to_string(), serde_json::json!(message));
            let fields = serde_json::Value::Object(map);
            ::spectra_core::try_log_event_now("valence_error_log", &fields);
        }
        pub fn log(
            source: String,
            operation: String,
            table: String,
            database_type: String,
            message: String,
        ) {
            Self::log_at(
                source,
                operation,
                table,
                database_type,
                message,
                chrono::Utc::now(),
            );
        }
    }
    #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
    pub struct ValenceOwnershipLog {
        pub table: String,
        pub record_id: String,
        pub from_owner_id: String,
        pub from_owner_type: String,
        pub to_owner_id: String,
        pub to_owner_type: String,
        pub actor: String,
    }
    pub struct ValenceOwnershipLogLogger;
    impl ValenceOwnershipLogLogger {
        #[doc = r" Emit with an explicit timestamp (preserved through buffered drain)."]
        pub fn log_at(
            table: String,
            record_id: String,
            from_owner_id: String,
            from_owner_type: String,
            to_owner_id: String,
            to_owner_type: String,
            actor: String,
            _ts: chrono::DateTime<chrono::Utc>,
        ) {
            let mut map = serde_json::Map::new();
            map.insert("table".to_string(), serde_json::json!(table));
            map.insert("record_id".to_string(), serde_json::json!(record_id));
            map.insert(
                "from_owner_id".to_string(),
                serde_json::json!(from_owner_id),
            );
            map.insert(
                "from_owner_type".to_string(),
                serde_json::json!(from_owner_type),
            );
            map.insert("to_owner_id".to_string(), serde_json::json!(to_owner_id));
            map.insert(
                "to_owner_type".to_string(),
                serde_json::json!(to_owner_type),
            );
            map.insert("actor".to_string(), serde_json::json!(actor));
            let fields = serde_json::Value::Object(map);
            ::spectra_core::try_log_event_now("valence_ownership_log", &fields);
        }
        pub fn log(
            table: String,
            record_id: String,
            from_owner_id: String,
            from_owner_type: String,
            to_owner_id: String,
            to_owner_type: String,
            actor: String,
        ) {
            Self::log_at(
                table,
                record_id,
                from_owner_id,
                from_owner_type,
                to_owner_id,
                to_owner_type,
                actor,
                chrono::Utc::now(),
            );
        }
    }
    #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
    pub struct ValencePrivacyEval {
        pub policy: String,
        pub actor_kind: String,
        pub actor_id: String,
        pub operation: String,
        pub table: String,
        pub rule_phase: String,
        pub matched: String,
        pub check_outcome: String,
    }
    pub struct ValencePrivacyEvalLogger;
    impl ValencePrivacyEvalLogger {
        #[doc = r" Emit with an explicit timestamp (preserved through buffered drain)."]
        pub fn log_at(
            policy: String,
            actor_kind: String,
            actor_id: String,
            operation: String,
            table: String,
            rule_phase: String,
            matched: String,
            check_outcome: String,
            _ts: chrono::DateTime<chrono::Utc>,
        ) {
            let mut map = serde_json::Map::new();
            map.insert("policy".to_string(), serde_json::json!(policy));
            map.insert("actor_kind".to_string(), serde_json::json!(actor_kind));
            map.insert("actor_id".to_string(), serde_json::json!(actor_id));
            map.insert("operation".to_string(), serde_json::json!(operation));
            map.insert("table".to_string(), serde_json::json!(table));
            map.insert("rule_phase".to_string(), serde_json::json!(rule_phase));
            map.insert("matched".to_string(), serde_json::json!(matched));
            map.insert(
                "check_outcome".to_string(),
                serde_json::json!(check_outcome),
            );
            let fields = serde_json::Value::Object(map);
            ::spectra_core::try_log_event_now("valence_privacy_eval", &fields);
        }
        pub fn log(
            policy: String,
            actor_kind: String,
            actor_id: String,
            operation: String,
            table: String,
            rule_phase: String,
            matched: String,
            check_outcome: String,
        ) {
            Self::log_at(
                policy,
                actor_kind,
                actor_id,
                operation,
                table,
                rule_phase,
                matched,
                check_outcome,
                chrono::Utc::now(),
            );
        }
    }
    #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
    pub struct ValenceQueryLog {
        pub outcome: String,
        pub query_target: String,
        pub trait_name: String,
        pub primary_table: String,
        pub implementor_count: String,
        pub hop_count: String,
        pub has_limit: String,
        pub limit: String,
        pub offset: String,
        pub rows_db: String,
        pub rows_after_privacy: String,
        pub rows_after_pending_deletion: String,
        pub wall_ms: String,
        pub compile_error: String,
        pub caller: String,
    }
    pub struct ValenceQueryLogLogger;
    impl ValenceQueryLogLogger {
        #[doc = r" Emit with an explicit timestamp (preserved through buffered drain)."]
        pub fn log_at(
            outcome: String,
            query_target: String,
            trait_name: String,
            primary_table: String,
            implementor_count: String,
            hop_count: String,
            has_limit: String,
            limit: String,
            offset: String,
            rows_db: String,
            rows_after_privacy: String,
            rows_after_pending_deletion: String,
            wall_ms: String,
            compile_error: String,
            caller: String,
            _ts: chrono::DateTime<chrono::Utc>,
        ) {
            let mut map = serde_json::Map::new();
            map.insert("outcome".to_string(), serde_json::json!(outcome));
            map.insert("query_target".to_string(), serde_json::json!(query_target));
            map.insert("trait_name".to_string(), serde_json::json!(trait_name));
            map.insert(
                "primary_table".to_string(),
                serde_json::json!(primary_table),
            );
            map.insert(
                "implementor_count".to_string(),
                serde_json::json!(implementor_count),
            );
            map.insert("hop_count".to_string(), serde_json::json!(hop_count));
            map.insert("has_limit".to_string(), serde_json::json!(has_limit));
            map.insert("limit".to_string(), serde_json::json!(limit));
            map.insert("offset".to_string(), serde_json::json!(offset));
            map.insert("rows_db".to_string(), serde_json::json!(rows_db));
            map.insert(
                "rows_after_privacy".to_string(),
                serde_json::json!(rows_after_privacy),
            );
            map.insert(
                "rows_after_pending_deletion".to_string(),
                serde_json::json!(rows_after_pending_deletion),
            );
            map.insert("wall_ms".to_string(), serde_json::json!(wall_ms));
            map.insert(
                "compile_error".to_string(),
                serde_json::json!(compile_error),
            );
            map.insert("caller".to_string(), serde_json::json!(caller));
            let fields = serde_json::Value::Object(map);
            ::spectra_core::try_log_event_now("valence_query_log", &fields);
        }
        pub fn log(
            outcome: String,
            query_target: String,
            trait_name: String,
            primary_table: String,
            implementor_count: String,
            hop_count: String,
            has_limit: String,
            limit: String,
            offset: String,
            rows_db: String,
            rows_after_privacy: String,
            rows_after_pending_deletion: String,
            wall_ms: String,
            compile_error: String,
            caller: String,
        ) {
            Self::log_at(
                outcome,
                query_target,
                trait_name,
                primary_table,
                implementor_count,
                hop_count,
                has_limit,
                limit,
                offset,
                rows_db,
                rows_after_privacy,
                rows_after_pending_deletion,
                wall_ms,
                compile_error,
                caller,
                chrono::Utc::now(),
            );
        }
    }
    #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
    pub struct ValenceSlowOp {
        pub operation: String,
        pub table: String,
        pub op: String,
        pub database_type: String,
        pub wall_ms: String,
        pub record_id: String,
    }
    pub struct ValenceSlowOpLogger;
    impl ValenceSlowOpLogger {
        #[doc = r" Emit with an explicit timestamp (preserved through buffered drain)."]
        pub fn log_at(
            operation: String,
            table: String,
            op: String,
            database_type: String,
            wall_ms: String,
            record_id: String,
            _ts: chrono::DateTime<chrono::Utc>,
        ) {
            let mut map = serde_json::Map::new();
            map.insert("operation".to_string(), serde_json::json!(operation));
            map.insert("table".to_string(), serde_json::json!(table));
            map.insert("op".to_string(), serde_json::json!(op));
            map.insert(
                "database_type".to_string(),
                serde_json::json!(database_type),
            );
            map.insert("wall_ms".to_string(), serde_json::json!(wall_ms));
            map.insert("record_id".to_string(), serde_json::json!(record_id));
            let fields = serde_json::Value::Object(map);
            ::spectra_core::try_log_event_now("valence_slow_op", &fields);
        }
        pub fn log(
            operation: String,
            table: String,
            op: String,
            database_type: String,
            wall_ms: String,
            record_id: String,
        ) {
            Self::log_at(
                operation,
                table,
                op,
                database_type,
                wall_ms,
                record_id,
                chrono::Utc::now(),
            );
        }
    }
    pub struct ValenceDbErrorsRecorder;
    impl ValenceDbErrorsRecorder {
        #[doc = r" Record with an explicit emit-time timestamp."]
        pub fn record_at(
            delta: i64,
            labels: serde_json::Value,
            _ts: chrono::DateTime<chrono::Utc>,
        ) {
            let label_pairs: Vec<(&str, &str)> = labels
                .as_object()
                .map(|obj| {
                    obj.iter()
                        .map(|(k, v)| (k.as_str(), v.as_str().unwrap_or("")))
                        .collect()
                })
                .unwrap_or_default();
            ::spectra_core::try_record_counter_now("valence_db_errors", &label_pairs, delta);
        }
        pub fn record(delta: i64, labels: serde_json::Value) {
            Self::record_at(delta, labels, chrono::Utc::now());
        }
    }
    pub struct ValenceDbReadsRecorder;
    impl ValenceDbReadsRecorder {
        #[doc = r" Record with an explicit emit-time timestamp."]
        pub fn record_at(
            delta: i64,
            labels: serde_json::Value,
            _ts: chrono::DateTime<chrono::Utc>,
        ) {
            let label_pairs: Vec<(&str, &str)> = labels
                .as_object()
                .map(|obj| {
                    obj.iter()
                        .map(|(k, v)| (k.as_str(), v.as_str().unwrap_or("")))
                        .collect()
                })
                .unwrap_or_default();
            ::spectra_core::try_record_counter_now("valence_db_reads", &label_pairs, delta);
        }
        pub fn record(delta: i64, labels: serde_json::Value) {
            Self::record_at(delta, labels, chrono::Utc::now());
        }
    }
    pub struct ValenceDbWallMsRecorder;
    impl ValenceDbWallMsRecorder {
        #[doc = r" Record with an explicit emit-time timestamp."]
        pub fn record_at(
            delta: i64,
            labels: serde_json::Value,
            _ts: chrono::DateTime<chrono::Utc>,
        ) {
            let label_pairs: Vec<(&str, &str)> = labels
                .as_object()
                .map(|obj| {
                    obj.iter()
                        .map(|(k, v)| (k.as_str(), v.as_str().unwrap_or("")))
                        .collect()
                })
                .unwrap_or_default();
            ::spectra_core::try_record_counter_now("valence_db_wall_ms", &label_pairs, delta);
        }
        pub fn record(delta: i64, labels: serde_json::Value) {
            Self::record_at(delta, labels, chrono::Utc::now());
        }
    }
    pub struct ValenceDbWritesRecorder;
    impl ValenceDbWritesRecorder {
        #[doc = r" Record with an explicit emit-time timestamp."]
        pub fn record_at(
            delta: i64,
            labels: serde_json::Value,
            _ts: chrono::DateTime<chrono::Utc>,
        ) {
            let label_pairs: Vec<(&str, &str)> = labels
                .as_object()
                .map(|obj| {
                    obj.iter()
                        .map(|(k, v)| (k.as_str(), v.as_str().unwrap_or("")))
                        .collect()
                })
                .unwrap_or_default();
            ::spectra_core::try_record_counter_now("valence_db_writes", &label_pairs, delta);
        }
        pub fn record(delta: i64, labels: serde_json::Value) {
            Self::record_at(delta, labels, chrono::Utc::now());
        }
    }
    pub struct ValenceDeletionDagMaxDepthRecorder;
    impl ValenceDeletionDagMaxDepthRecorder {
        #[doc = r" Record with an explicit emit-time timestamp."]
        pub fn record_at(
            delta: i64,
            labels: serde_json::Value,
            _ts: chrono::DateTime<chrono::Utc>,
        ) {
            let label_pairs: Vec<(&str, &str)> = labels
                .as_object()
                .map(|obj| {
                    obj.iter()
                        .map(|(k, v)| (k.as_str(), v.as_str().unwrap_or("")))
                        .collect()
                })
                .unwrap_or_default();
            ::spectra_core::try_record_counter_now(
                "valence_deletion_dag_max_depth",
                &label_pairs,
                delta,
            );
        }
        pub fn record(delta: i64, labels: serde_json::Value) {
            Self::record_at(delta, labels, chrono::Utc::now());
        }
    }
    pub struct ValenceDeletionDagNodesRecorder;
    impl ValenceDeletionDagNodesRecorder {
        #[doc = r" Record with an explicit emit-time timestamp."]
        pub fn record_at(
            delta: i64,
            labels: serde_json::Value,
            _ts: chrono::DateTime<chrono::Utc>,
        ) {
            let label_pairs: Vec<(&str, &str)> = labels
                .as_object()
                .map(|obj| {
                    obj.iter()
                        .map(|(k, v)| (k.as_str(), v.as_str().unwrap_or("")))
                        .collect()
                })
                .unwrap_or_default();
            ::spectra_core::try_record_counter_now(
                "valence_deletion_dag_nodes",
                &label_pairs,
                delta,
            );
        }
        pub fn record(delta: i64, labels: serde_json::Value) {
            Self::record_at(delta, labels, chrono::Utc::now());
        }
    }
    pub struct ValenceDeletionRestrictBlockedRecorder;
    impl ValenceDeletionRestrictBlockedRecorder {
        #[doc = r" Record with an explicit emit-time timestamp."]
        pub fn record_at(
            delta: i64,
            labels: serde_json::Value,
            _ts: chrono::DateTime<chrono::Utc>,
        ) {
            let label_pairs: Vec<(&str, &str)> = labels
                .as_object()
                .map(|obj| {
                    obj.iter()
                        .map(|(k, v)| (k.as_str(), v.as_str().unwrap_or("")))
                        .collect()
                })
                .unwrap_or_default();
            ::spectra_core::try_record_counter_now(
                "valence_deletion_restrict_blocked",
                &label_pairs,
                delta,
            );
        }
        pub fn record(delta: i64, labels: serde_json::Value) {
            Self::record_at(delta, labels, chrono::Utc::now());
        }
    }
    pub struct ValenceDeletionRunQueuedRecorder;
    impl ValenceDeletionRunQueuedRecorder {
        #[doc = r" Record with an explicit emit-time timestamp."]
        pub fn record_at(
            delta: i64,
            labels: serde_json::Value,
            _ts: chrono::DateTime<chrono::Utc>,
        ) {
            let label_pairs: Vec<(&str, &str)> = labels
                .as_object()
                .map(|obj| {
                    obj.iter()
                        .map(|(k, v)| (k.as_str(), v.as_str().unwrap_or("")))
                        .collect()
                })
                .unwrap_or_default();
            ::spectra_core::try_record_counter_now(
                "valence_deletion_run_queued",
                &label_pairs,
                delta,
            );
        }
        pub fn record(delta: i64, labels: serde_json::Value) {
            Self::record_at(delta, labels, chrono::Utc::now());
        }
    }
    pub struct ValenceDeserializeErrorsRecorder;
    impl ValenceDeserializeErrorsRecorder {
        #[doc = r" Record with an explicit emit-time timestamp."]
        pub fn record_at(
            delta: i64,
            labels: serde_json::Value,
            _ts: chrono::DateTime<chrono::Utc>,
        ) {
            let label_pairs: Vec<(&str, &str)> = labels
                .as_object()
                .map(|obj| {
                    obj.iter()
                        .map(|(k, v)| (k.as_str(), v.as_str().unwrap_or("")))
                        .collect()
                })
                .unwrap_or_default();
            ::spectra_core::try_record_counter_now(
                "valence_deserialize_errors",
                &label_pairs,
                delta,
            );
        }
        pub fn record(delta: i64, labels: serde_json::Value) {
            Self::record_at(delta, labels, chrono::Utc::now());
        }
    }
    pub struct ValenceEdgeReadsRecorder;
    impl ValenceEdgeReadsRecorder {
        #[doc = r" Record with an explicit emit-time timestamp."]
        pub fn record_at(
            delta: i64,
            labels: serde_json::Value,
            _ts: chrono::DateTime<chrono::Utc>,
        ) {
            let label_pairs: Vec<(&str, &str)> = labels
                .as_object()
                .map(|obj| {
                    obj.iter()
                        .map(|(k, v)| (k.as_str(), v.as_str().unwrap_or("")))
                        .collect()
                })
                .unwrap_or_default();
            ::spectra_core::try_record_counter_now("valence_edge_reads", &label_pairs, delta);
        }
        pub fn record(delta: i64, labels: serde_json::Value) {
            Self::record_at(delta, labels, chrono::Utc::now());
        }
    }
    pub struct ValenceEdgeWritesRecorder;
    impl ValenceEdgeWritesRecorder {
        #[doc = r" Record with an explicit emit-time timestamp."]
        pub fn record_at(
            delta: i64,
            labels: serde_json::Value,
            _ts: chrono::DateTime<chrono::Utc>,
        ) {
            let label_pairs: Vec<(&str, &str)> = labels
                .as_object()
                .map(|obj| {
                    obj.iter()
                        .map(|(k, v)| (k.as_str(), v.as_str().unwrap_or("")))
                        .collect()
                })
                .unwrap_or_default();
            ::spectra_core::try_record_counter_now("valence_edge_writes", &label_pairs, delta);
        }
        pub fn record(delta: i64, labels: serde_json::Value) {
            Self::record_at(delta, labels, chrono::Utc::now());
        }
    }
    pub struct ValenceFieldRedactionsRecorder;
    impl ValenceFieldRedactionsRecorder {
        #[doc = r" Record with an explicit emit-time timestamp."]
        pub fn record_at(
            delta: i64,
            labels: serde_json::Value,
            _ts: chrono::DateTime<chrono::Utc>,
        ) {
            let label_pairs: Vec<(&str, &str)> = labels
                .as_object()
                .map(|obj| {
                    obj.iter()
                        .map(|(k, v)| (k.as_str(), v.as_str().unwrap_or("")))
                        .collect()
                })
                .unwrap_or_default();
            ::spectra_core::try_record_counter_now("valence_field_redactions", &label_pairs, delta);
        }
        pub fn record(delta: i64, labels: serde_json::Value) {
            Self::record_at(delta, labels, chrono::Utc::now());
        }
    }
    pub struct ValenceMutationWallMsRecorder;
    impl ValenceMutationWallMsRecorder {
        #[doc = r" Record with an explicit emit-time timestamp."]
        pub fn record_at(
            delta: i64,
            labels: serde_json::Value,
            _ts: chrono::DateTime<chrono::Utc>,
        ) {
            let label_pairs: Vec<(&str, &str)> = labels
                .as_object()
                .map(|obj| {
                    obj.iter()
                        .map(|(k, v)| (k.as_str(), v.as_str().unwrap_or("")))
                        .collect()
                })
                .unwrap_or_default();
            ::spectra_core::try_record_counter_now("valence_mutation_wall_ms", &label_pairs, delta);
        }
        pub fn record(delta: i64, labels: serde_json::Value) {
            Self::record_at(delta, labels, chrono::Utc::now());
        }
    }
    pub struct ValenceOwnershipTransfersRecorder;
    impl ValenceOwnershipTransfersRecorder {
        #[doc = r" Record with an explicit emit-time timestamp."]
        pub fn record_at(
            delta: i64,
            labels: serde_json::Value,
            _ts: chrono::DateTime<chrono::Utc>,
        ) {
            let label_pairs: Vec<(&str, &str)> = labels
                .as_object()
                .map(|obj| {
                    obj.iter()
                        .map(|(k, v)| (k.as_str(), v.as_str().unwrap_or("")))
                        .collect()
                })
                .unwrap_or_default();
            ::spectra_core::try_record_counter_now(
                "valence_ownership_transfers",
                &label_pairs,
                delta,
            );
        }
        pub fn record(delta: i64, labels: serde_json::Value) {
            Self::record_at(delta, labels, chrono::Utc::now());
        }
    }
    pub struct ValencePrivacyDenialsRecorder;
    impl ValencePrivacyDenialsRecorder {
        #[doc = r" Record with an explicit emit-time timestamp."]
        pub fn record_at(
            delta: i64,
            labels: serde_json::Value,
            _ts: chrono::DateTime<chrono::Utc>,
        ) {
            let label_pairs: Vec<(&str, &str)> = labels
                .as_object()
                .map(|obj| {
                    obj.iter()
                        .map(|(k, v)| (k.as_str(), v.as_str().unwrap_or("")))
                        .collect()
                })
                .unwrap_or_default();
            ::spectra_core::try_record_counter_now("valence_privacy_denials", &label_pairs, delta);
        }
        pub fn record(delta: i64, labels: serde_json::Value) {
            Self::record_at(delta, labels, chrono::Utc::now());
        }
    }
    pub struct ValencePrivacyEvaluationsRecorder;
    impl ValencePrivacyEvaluationsRecorder {
        #[doc = r" Record with an explicit emit-time timestamp."]
        pub fn record_at(
            delta: i64,
            labels: serde_json::Value,
            _ts: chrono::DateTime<chrono::Utc>,
        ) {
            let label_pairs: Vec<(&str, &str)> = labels
                .as_object()
                .map(|obj| {
                    obj.iter()
                        .map(|(k, v)| (k.as_str(), v.as_str().unwrap_or("")))
                        .collect()
                })
                .unwrap_or_default();
            ::spectra_core::try_record_counter_now(
                "valence_privacy_evaluations",
                &label_pairs,
                delta,
            );
        }
        pub fn record(delta: i64, labels: serde_json::Value) {
            Self::record_at(delta, labels, chrono::Utc::now());
        }
    }
    pub struct ValenceQueryCompileErrorsRecorder;
    impl ValenceQueryCompileErrorsRecorder {
        #[doc = r" Record with an explicit emit-time timestamp."]
        pub fn record_at(
            delta: i64,
            labels: serde_json::Value,
            _ts: chrono::DateTime<chrono::Utc>,
        ) {
            let label_pairs: Vec<(&str, &str)> = labels
                .as_object()
                .map(|obj| {
                    obj.iter()
                        .map(|(k, v)| (k.as_str(), v.as_str().unwrap_or("")))
                        .collect()
                })
                .unwrap_or_default();
            ::spectra_core::try_record_counter_now(
                "valence_query_compile_errors",
                &label_pairs,
                delta,
            );
        }
        pub fn record(delta: i64, labels: serde_json::Value) {
            Self::record_at(delta, labels, chrono::Utc::now());
        }
    }
    pub struct ValenceQueryExecutionsRecorder;
    impl ValenceQueryExecutionsRecorder {
        #[doc = r" Record with an explicit emit-time timestamp."]
        pub fn record_at(
            delta: i64,
            labels: serde_json::Value,
            _ts: chrono::DateTime<chrono::Utc>,
        ) {
            let label_pairs: Vec<(&str, &str)> = labels
                .as_object()
                .map(|obj| {
                    obj.iter()
                        .map(|(k, v)| (k.as_str(), v.as_str().unwrap_or("")))
                        .collect()
                })
                .unwrap_or_default();
            ::spectra_core::try_record_counter_now("valence_query_executions", &label_pairs, delta);
        }
        pub fn record(delta: i64, labels: serde_json::Value) {
            Self::record_at(delta, labels, chrono::Utc::now());
        }
    }
    pub struct ValenceQueryResultsCappedRecorder;
    impl ValenceQueryResultsCappedRecorder {
        #[doc = r" Record with an explicit emit-time timestamp."]
        pub fn record_at(
            delta: i64,
            labels: serde_json::Value,
            _ts: chrono::DateTime<chrono::Utc>,
        ) {
            let label_pairs: Vec<(&str, &str)> = labels
                .as_object()
                .map(|obj| {
                    obj.iter()
                        .map(|(k, v)| (k.as_str(), v.as_str().unwrap_or("")))
                        .collect()
                })
                .unwrap_or_default();
            ::spectra_core::try_record_counter_now(
                "valence_query_results_capped",
                &label_pairs,
                delta,
            );
        }
        pub fn record(delta: i64, labels: serde_json::Value) {
            Self::record_at(delta, labels, chrono::Utc::now());
        }
    }
    pub struct ValenceQueryRowsFilteredRecorder;
    impl ValenceQueryRowsFilteredRecorder {
        #[doc = r" Record with an explicit emit-time timestamp."]
        pub fn record_at(
            delta: i64,
            labels: serde_json::Value,
            _ts: chrono::DateTime<chrono::Utc>,
        ) {
            let label_pairs: Vec<(&str, &str)> = labels
                .as_object()
                .map(|obj| {
                    obj.iter()
                        .map(|(k, v)| (k.as_str(), v.as_str().unwrap_or("")))
                        .collect()
                })
                .unwrap_or_default();
            ::spectra_core::try_record_counter_now(
                "valence_query_rows_filtered",
                &label_pairs,
                delta,
            );
        }
        pub fn record(delta: i64, labels: serde_json::Value) {
            Self::record_at(delta, labels, chrono::Utc::now());
        }
    }
    pub struct ValenceQueryWallMsRecorder;
    impl ValenceQueryWallMsRecorder {
        #[doc = r" Record with an explicit emit-time timestamp."]
        pub fn record_at(
            delta: i64,
            labels: serde_json::Value,
            _ts: chrono::DateTime<chrono::Utc>,
        ) {
            let label_pairs: Vec<(&str, &str)> = labels
                .as_object()
                .map(|obj| {
                    obj.iter()
                        .map(|(k, v)| (k.as_str(), v.as_str().unwrap_or("")))
                        .collect()
                })
                .unwrap_or_default();
            ::spectra_core::try_record_counter_now("valence_query_wall_ms", &label_pairs, delta);
        }
        pub fn record(delta: i64, labels: serde_json::Value) {
            Self::record_at(delta, labels, chrono::Utc::now());
        }
    }
    pub struct ValenceRouterResolveErrorsRecorder;
    impl ValenceRouterResolveErrorsRecorder {
        #[doc = r" Record with an explicit emit-time timestamp."]
        pub fn record_at(
            delta: i64,
            labels: serde_json::Value,
            _ts: chrono::DateTime<chrono::Utc>,
        ) {
            let label_pairs: Vec<(&str, &str)> = labels
                .as_object()
                .map(|obj| {
                    obj.iter()
                        .map(|(k, v)| (k.as_str(), v.as_str().unwrap_or("")))
                        .collect()
                })
                .unwrap_or_default();
            ::spectra_core::try_record_counter_now(
                "valence_router_resolve_errors",
                &label_pairs,
                delta,
            );
        }
        pub fn record(delta: i64, labels: serde_json::Value) {
            Self::record_at(delta, labels, chrono::Utc::now());
        }
    }
    pub struct ValenceSideEffectsRecorder;
    impl ValenceSideEffectsRecorder {
        #[doc = r" Record with an explicit emit-time timestamp."]
        pub fn record_at(
            delta: i64,
            labels: serde_json::Value,
            _ts: chrono::DateTime<chrono::Utc>,
        ) {
            let label_pairs: Vec<(&str, &str)> = labels
                .as_object()
                .map(|obj| {
                    obj.iter()
                        .map(|(k, v)| (k.as_str(), v.as_str().unwrap_or("")))
                        .collect()
                })
                .unwrap_or_default();
            ::spectra_core::try_record_counter_now("valence_side_effects", &label_pairs, delta);
        }
        pub fn record(delta: i64, labels: serde_json::Value) {
            Self::record_at(delta, labels, chrono::Utc::now());
        }
    }
    pub struct ValenceUniqueViolationsRecorder;
    impl ValenceUniqueViolationsRecorder {
        #[doc = r" Record with an explicit emit-time timestamp."]
        pub fn record_at(
            delta: i64,
            labels: serde_json::Value,
            _ts: chrono::DateTime<chrono::Utc>,
        ) {
            let label_pairs: Vec<(&str, &str)> = labels
                .as_object()
                .map(|obj| {
                    obj.iter()
                        .map(|(k, v)| (k.as_str(), v.as_str().unwrap_or("")))
                        .collect()
                })
                .unwrap_or_default();
            ::spectra_core::try_record_counter_now(
                "valence_unique_violations",
                &label_pairs,
                delta,
            );
        }
        pub fn record(delta: i64, labels: serde_json::Value) {
            Self::record_at(delta, labels, chrono::Utc::now());
        }
    }
    pub struct ValenceValidationFailuresRecorder;
    impl ValenceValidationFailuresRecorder {
        #[doc = r" Record with an explicit emit-time timestamp."]
        pub fn record_at(
            delta: i64,
            labels: serde_json::Value,
            _ts: chrono::DateTime<chrono::Utc>,
        ) {
            let label_pairs: Vec<(&str, &str)> = labels
                .as_object()
                .map(|obj| {
                    obj.iter()
                        .map(|(k, v)| (k.as_str(), v.as_str().unwrap_or("")))
                        .collect()
                })
                .unwrap_or_default();
            ::spectra_core::try_record_counter_now(
                "valence_validation_failures",
                &label_pairs,
                delta,
            );
        }
        pub fn record(delta: i64, labels: serde_json::Value) {
            Self::record_at(delta, labels, chrono::Utc::now());
        }
    }
}
pub mod topics {
    pub const VALENCE_DELETION_LOG_TOPIC: &str = "spectra.event.valence_deletion_log";
    #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
    pub struct ValenceDeletionLogPayload {
        pub schema_table: &'static str,
        pub outcome: String,
        pub root_table: String,
        pub root_record_id: String,
        pub node_count: String,
        pub max_depth: String,
        pub restrict_count: String,
        pub trait_expansion_count: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ts: Option<chrono::DateTime<chrono::Utc>>,
    }
    impl ValenceDeletionLogPayload {
        pub fn topic() -> &'static str {
            VALENCE_DELETION_LOG_TOPIC
        }
        pub fn to_spectra_event(&self) -> ::spectra_core::SpectraEvent {
            let mut map = serde_json::Map::new();
            map.insert("outcome".to_string(), serde_json::json!(self.outcome));
            map.insert("root_table".to_string(), serde_json::json!(self.root_table));
            map.insert(
                "root_record_id".to_string(),
                serde_json::json!(self.root_record_id),
            );
            map.insert("node_count".to_string(), serde_json::json!(self.node_count));
            map.insert("max_depth".to_string(), serde_json::json!(self.max_depth));
            map.insert(
                "restrict_count".to_string(),
                serde_json::json!(self.restrict_count),
            );
            map.insert(
                "trait_expansion_count".to_string(),
                serde_json::json!(self.trait_expansion_count),
            );
            let fields = serde_json::Value::Object(map);
            match self.ts {
                Some(ts) => {
                    ::spectra_core::SpectraEvent::with_ts("valence_deletion_log", fields, ts)
                }
                None => ::spectra_core::SpectraEvent::new("valence_deletion_log", fields),
            }
        }
    }
    pub const VALENCE_ERROR_LOG_TOPIC: &str = "spectra.event.valence_error_log";
    #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
    pub struct ValenceErrorLogPayload {
        pub schema_table: &'static str,
        pub source: String,
        pub operation: String,
        pub table: String,
        pub database_type: String,
        pub message: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ts: Option<chrono::DateTime<chrono::Utc>>,
    }
    impl ValenceErrorLogPayload {
        pub fn topic() -> &'static str {
            VALENCE_ERROR_LOG_TOPIC
        }
        pub fn to_spectra_event(&self) -> ::spectra_core::SpectraEvent {
            let mut map = serde_json::Map::new();
            map.insert("source".to_string(), serde_json::json!(self.source));
            map.insert("operation".to_string(), serde_json::json!(self.operation));
            map.insert("table".to_string(), serde_json::json!(self.table));
            map.insert(
                "database_type".to_string(),
                serde_json::json!(self.database_type),
            );
            map.insert("message".to_string(), serde_json::json!(self.message));
            let fields = serde_json::Value::Object(map);
            match self.ts {
                Some(ts) => ::spectra_core::SpectraEvent::with_ts("valence_error_log", fields, ts),
                None => ::spectra_core::SpectraEvent::new("valence_error_log", fields),
            }
        }
    }
    pub const VALENCE_OWNERSHIP_LOG_TOPIC: &str = "spectra.event.valence_ownership_log";
    #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
    pub struct ValenceOwnershipLogPayload {
        pub schema_table: &'static str,
        pub table: String,
        pub record_id: String,
        pub from_owner_id: String,
        pub from_owner_type: String,
        pub to_owner_id: String,
        pub to_owner_type: String,
        pub actor: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ts: Option<chrono::DateTime<chrono::Utc>>,
    }
    impl ValenceOwnershipLogPayload {
        pub fn topic() -> &'static str {
            VALENCE_OWNERSHIP_LOG_TOPIC
        }
        pub fn to_spectra_event(&self) -> ::spectra_core::SpectraEvent {
            let mut map = serde_json::Map::new();
            map.insert("table".to_string(), serde_json::json!(self.table));
            map.insert("record_id".to_string(), serde_json::json!(self.record_id));
            map.insert(
                "from_owner_id".to_string(),
                serde_json::json!(self.from_owner_id),
            );
            map.insert(
                "from_owner_type".to_string(),
                serde_json::json!(self.from_owner_type),
            );
            map.insert(
                "to_owner_id".to_string(),
                serde_json::json!(self.to_owner_id),
            );
            map.insert(
                "to_owner_type".to_string(),
                serde_json::json!(self.to_owner_type),
            );
            map.insert("actor".to_string(), serde_json::json!(self.actor));
            let fields = serde_json::Value::Object(map);
            match self.ts {
                Some(ts) => {
                    ::spectra_core::SpectraEvent::with_ts("valence_ownership_log", fields, ts)
                }
                None => ::spectra_core::SpectraEvent::new("valence_ownership_log", fields),
            }
        }
    }
    pub const VALENCE_PRIVACY_EVAL_TOPIC: &str = "spectra.event.valence_privacy_eval";
    #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
    pub struct ValencePrivacyEvalPayload {
        pub schema_table: &'static str,
        pub policy: String,
        pub actor_kind: String,
        pub actor_id: String,
        pub operation: String,
        pub table: String,
        pub rule_phase: String,
        pub matched: String,
        pub check_outcome: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ts: Option<chrono::DateTime<chrono::Utc>>,
    }
    impl ValencePrivacyEvalPayload {
        pub fn topic() -> &'static str {
            VALENCE_PRIVACY_EVAL_TOPIC
        }
        pub fn to_spectra_event(&self) -> ::spectra_core::SpectraEvent {
            let mut map = serde_json::Map::new();
            map.insert("policy".to_string(), serde_json::json!(self.policy));
            map.insert("actor_kind".to_string(), serde_json::json!(self.actor_kind));
            map.insert("actor_id".to_string(), serde_json::json!(self.actor_id));
            map.insert("operation".to_string(), serde_json::json!(self.operation));
            map.insert("table".to_string(), serde_json::json!(self.table));
            map.insert("rule_phase".to_string(), serde_json::json!(self.rule_phase));
            map.insert("matched".to_string(), serde_json::json!(self.matched));
            map.insert(
                "check_outcome".to_string(),
                serde_json::json!(self.check_outcome),
            );
            let fields = serde_json::Value::Object(map);
            match self.ts {
                Some(ts) => {
                    ::spectra_core::SpectraEvent::with_ts("valence_privacy_eval", fields, ts)
                }
                None => ::spectra_core::SpectraEvent::new("valence_privacy_eval", fields),
            }
        }
    }
    pub const VALENCE_QUERY_LOG_TOPIC: &str = "spectra.event.valence_query_log";
    #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
    pub struct ValenceQueryLogPayload {
        pub schema_table: &'static str,
        pub outcome: String,
        pub query_target: String,
        pub trait_name: String,
        pub primary_table: String,
        pub implementor_count: String,
        pub hop_count: String,
        pub has_limit: String,
        pub limit: String,
        pub offset: String,
        pub rows_db: String,
        pub rows_after_privacy: String,
        pub rows_after_pending_deletion: String,
        pub wall_ms: String,
        pub compile_error: String,
        pub caller: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ts: Option<chrono::DateTime<chrono::Utc>>,
    }
    impl ValenceQueryLogPayload {
        pub fn topic() -> &'static str {
            VALENCE_QUERY_LOG_TOPIC
        }
        pub fn to_spectra_event(&self) -> ::spectra_core::SpectraEvent {
            let mut map = serde_json::Map::new();
            map.insert("outcome".to_string(), serde_json::json!(self.outcome));
            map.insert(
                "query_target".to_string(),
                serde_json::json!(self.query_target),
            );
            map.insert("trait_name".to_string(), serde_json::json!(self.trait_name));
            map.insert(
                "primary_table".to_string(),
                serde_json::json!(self.primary_table),
            );
            map.insert(
                "implementor_count".to_string(),
                serde_json::json!(self.implementor_count),
            );
            map.insert("hop_count".to_string(), serde_json::json!(self.hop_count));
            map.insert("has_limit".to_string(), serde_json::json!(self.has_limit));
            map.insert("limit".to_string(), serde_json::json!(self.limit));
            map.insert("offset".to_string(), serde_json::json!(self.offset));
            map.insert("rows_db".to_string(), serde_json::json!(self.rows_db));
            map.insert(
                "rows_after_privacy".to_string(),
                serde_json::json!(self.rows_after_privacy),
            );
            map.insert(
                "rows_after_pending_deletion".to_string(),
                serde_json::json!(self.rows_after_pending_deletion),
            );
            map.insert("wall_ms".to_string(), serde_json::json!(self.wall_ms));
            map.insert(
                "compile_error".to_string(),
                serde_json::json!(self.compile_error),
            );
            map.insert("caller".to_string(), serde_json::json!(self.caller));
            let fields = serde_json::Value::Object(map);
            match self.ts {
                Some(ts) => ::spectra_core::SpectraEvent::with_ts("valence_query_log", fields, ts),
                None => ::spectra_core::SpectraEvent::new("valence_query_log", fields),
            }
        }
    }
    pub const VALENCE_SLOW_OP_TOPIC: &str = "spectra.event.valence_slow_op";
    #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
    pub struct ValenceSlowOpPayload {
        pub schema_table: &'static str,
        pub operation: String,
        pub table: String,
        pub op: String,
        pub database_type: String,
        pub wall_ms: String,
        pub record_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ts: Option<chrono::DateTime<chrono::Utc>>,
    }
    impl ValenceSlowOpPayload {
        pub fn topic() -> &'static str {
            VALENCE_SLOW_OP_TOPIC
        }
        pub fn to_spectra_event(&self) -> ::spectra_core::SpectraEvent {
            let mut map = serde_json::Map::new();
            map.insert("operation".to_string(), serde_json::json!(self.operation));
            map.insert("table".to_string(), serde_json::json!(self.table));
            map.insert("op".to_string(), serde_json::json!(self.op));
            map.insert(
                "database_type".to_string(),
                serde_json::json!(self.database_type),
            );
            map.insert("wall_ms".to_string(), serde_json::json!(self.wall_ms));
            map.insert("record_id".to_string(), serde_json::json!(self.record_id));
            let fields = serde_json::Value::Object(map);
            match self.ts {
                Some(ts) => ::spectra_core::SpectraEvent::with_ts("valence_slow_op", fields, ts),
                None => ::spectra_core::SpectraEvent::new("valence_slow_op", fields),
            }
        }
    }
    pub const VALENCE_DB_ERRORS_TOPIC: &str = "spectra.metric.valence_db_errors";
    #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
    pub struct ValenceDbErrorsPayload {
        pub name: &'static str,
        pub labels: serde_json::Value,
        pub delta: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ts: Option<chrono::DateTime<chrono::Utc>>,
    }
    impl ValenceDbErrorsPayload {
        pub fn topic() -> &'static str {
            VALENCE_DB_ERRORS_TOPIC
        }
        pub fn to_metric_emit(&self) -> ::spectra_core::MetricEmit {
            match self.ts {
                Some(ts) => ::spectra_core::MetricEmit::counter(
                    "valence_db_errors",
                    self.labels.clone(),
                    self.delta,
                    ts,
                ),
                None => ::spectra_core::MetricEmit::counter(
                    "valence_db_errors",
                    self.labels.clone(),
                    self.delta,
                    chrono::Utc::now(),
                ),
            }
        }
    }
    pub const VALENCE_DB_READS_TOPIC: &str = "spectra.metric.valence_db_reads";
    #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
    pub struct ValenceDbReadsPayload {
        pub name: &'static str,
        pub labels: serde_json::Value,
        pub delta: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ts: Option<chrono::DateTime<chrono::Utc>>,
    }
    impl ValenceDbReadsPayload {
        pub fn topic() -> &'static str {
            VALENCE_DB_READS_TOPIC
        }
        pub fn to_metric_emit(&self) -> ::spectra_core::MetricEmit {
            match self.ts {
                Some(ts) => ::spectra_core::MetricEmit::counter(
                    "valence_db_reads",
                    self.labels.clone(),
                    self.delta,
                    ts,
                ),
                None => ::spectra_core::MetricEmit::counter(
                    "valence_db_reads",
                    self.labels.clone(),
                    self.delta,
                    chrono::Utc::now(),
                ),
            }
        }
    }
    pub const VALENCE_DB_WALL_MS_TOPIC: &str = "spectra.metric.valence_db_wall_ms";
    #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
    pub struct ValenceDbWallMsPayload {
        pub name: &'static str,
        pub labels: serde_json::Value,
        pub delta: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ts: Option<chrono::DateTime<chrono::Utc>>,
    }
    impl ValenceDbWallMsPayload {
        pub fn topic() -> &'static str {
            VALENCE_DB_WALL_MS_TOPIC
        }
        pub fn to_metric_emit(&self) -> ::spectra_core::MetricEmit {
            match self.ts {
                Some(ts) => ::spectra_core::MetricEmit::counter(
                    "valence_db_wall_ms",
                    self.labels.clone(),
                    self.delta,
                    ts,
                ),
                None => ::spectra_core::MetricEmit::counter(
                    "valence_db_wall_ms",
                    self.labels.clone(),
                    self.delta,
                    chrono::Utc::now(),
                ),
            }
        }
    }
    pub const VALENCE_DB_WRITES_TOPIC: &str = "spectra.metric.valence_db_writes";
    #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
    pub struct ValenceDbWritesPayload {
        pub name: &'static str,
        pub labels: serde_json::Value,
        pub delta: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ts: Option<chrono::DateTime<chrono::Utc>>,
    }
    impl ValenceDbWritesPayload {
        pub fn topic() -> &'static str {
            VALENCE_DB_WRITES_TOPIC
        }
        pub fn to_metric_emit(&self) -> ::spectra_core::MetricEmit {
            match self.ts {
                Some(ts) => ::spectra_core::MetricEmit::counter(
                    "valence_db_writes",
                    self.labels.clone(),
                    self.delta,
                    ts,
                ),
                None => ::spectra_core::MetricEmit::counter(
                    "valence_db_writes",
                    self.labels.clone(),
                    self.delta,
                    chrono::Utc::now(),
                ),
            }
        }
    }
    pub const VALENCE_DELETION_DAG_MAX_DEPTH_TOPIC: &str =
        "spectra.metric.valence_deletion_dag_max_depth";
    #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
    pub struct ValenceDeletionDagMaxDepthPayload {
        pub name: &'static str,
        pub labels: serde_json::Value,
        pub delta: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ts: Option<chrono::DateTime<chrono::Utc>>,
    }
    impl ValenceDeletionDagMaxDepthPayload {
        pub fn topic() -> &'static str {
            VALENCE_DELETION_DAG_MAX_DEPTH_TOPIC
        }
        pub fn to_metric_emit(&self) -> ::spectra_core::MetricEmit {
            match self.ts {
                Some(ts) => ::spectra_core::MetricEmit::counter(
                    "valence_deletion_dag_max_depth",
                    self.labels.clone(),
                    self.delta,
                    ts,
                ),
                None => ::spectra_core::MetricEmit::counter(
                    "valence_deletion_dag_max_depth",
                    self.labels.clone(),
                    self.delta,
                    chrono::Utc::now(),
                ),
            }
        }
    }
    pub const VALENCE_DELETION_DAG_NODES_TOPIC: &str = "spectra.metric.valence_deletion_dag_nodes";
    #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
    pub struct ValenceDeletionDagNodesPayload {
        pub name: &'static str,
        pub labels: serde_json::Value,
        pub delta: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ts: Option<chrono::DateTime<chrono::Utc>>,
    }
    impl ValenceDeletionDagNodesPayload {
        pub fn topic() -> &'static str {
            VALENCE_DELETION_DAG_NODES_TOPIC
        }
        pub fn to_metric_emit(&self) -> ::spectra_core::MetricEmit {
            match self.ts {
                Some(ts) => ::spectra_core::MetricEmit::counter(
                    "valence_deletion_dag_nodes",
                    self.labels.clone(),
                    self.delta,
                    ts,
                ),
                None => ::spectra_core::MetricEmit::counter(
                    "valence_deletion_dag_nodes",
                    self.labels.clone(),
                    self.delta,
                    chrono::Utc::now(),
                ),
            }
        }
    }
    pub const VALENCE_DELETION_RESTRICT_BLOCKED_TOPIC: &str =
        "spectra.metric.valence_deletion_restrict_blocked";
    #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
    pub struct ValenceDeletionRestrictBlockedPayload {
        pub name: &'static str,
        pub labels: serde_json::Value,
        pub delta: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ts: Option<chrono::DateTime<chrono::Utc>>,
    }
    impl ValenceDeletionRestrictBlockedPayload {
        pub fn topic() -> &'static str {
            VALENCE_DELETION_RESTRICT_BLOCKED_TOPIC
        }
        pub fn to_metric_emit(&self) -> ::spectra_core::MetricEmit {
            match self.ts {
                Some(ts) => ::spectra_core::MetricEmit::counter(
                    "valence_deletion_restrict_blocked",
                    self.labels.clone(),
                    self.delta,
                    ts,
                ),
                None => ::spectra_core::MetricEmit::counter(
                    "valence_deletion_restrict_blocked",
                    self.labels.clone(),
                    self.delta,
                    chrono::Utc::now(),
                ),
            }
        }
    }
    pub const VALENCE_DELETION_RUN_QUEUED_TOPIC: &str =
        "spectra.metric.valence_deletion_run_queued";
    #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
    pub struct ValenceDeletionRunQueuedPayload {
        pub name: &'static str,
        pub labels: serde_json::Value,
        pub delta: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ts: Option<chrono::DateTime<chrono::Utc>>,
    }
    impl ValenceDeletionRunQueuedPayload {
        pub fn topic() -> &'static str {
            VALENCE_DELETION_RUN_QUEUED_TOPIC
        }
        pub fn to_metric_emit(&self) -> ::spectra_core::MetricEmit {
            match self.ts {
                Some(ts) => ::spectra_core::MetricEmit::counter(
                    "valence_deletion_run_queued",
                    self.labels.clone(),
                    self.delta,
                    ts,
                ),
                None => ::spectra_core::MetricEmit::counter(
                    "valence_deletion_run_queued",
                    self.labels.clone(),
                    self.delta,
                    chrono::Utc::now(),
                ),
            }
        }
    }
    pub const VALENCE_DESERIALIZE_ERRORS_TOPIC: &str = "spectra.metric.valence_deserialize_errors";
    #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
    pub struct ValenceDeserializeErrorsPayload {
        pub name: &'static str,
        pub labels: serde_json::Value,
        pub delta: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ts: Option<chrono::DateTime<chrono::Utc>>,
    }
    impl ValenceDeserializeErrorsPayload {
        pub fn topic() -> &'static str {
            VALENCE_DESERIALIZE_ERRORS_TOPIC
        }
        pub fn to_metric_emit(&self) -> ::spectra_core::MetricEmit {
            match self.ts {
                Some(ts) => ::spectra_core::MetricEmit::counter(
                    "valence_deserialize_errors",
                    self.labels.clone(),
                    self.delta,
                    ts,
                ),
                None => ::spectra_core::MetricEmit::counter(
                    "valence_deserialize_errors",
                    self.labels.clone(),
                    self.delta,
                    chrono::Utc::now(),
                ),
            }
        }
    }
    pub const VALENCE_EDGE_READS_TOPIC: &str = "spectra.metric.valence_edge_reads";
    #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
    pub struct ValenceEdgeReadsPayload {
        pub name: &'static str,
        pub labels: serde_json::Value,
        pub delta: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ts: Option<chrono::DateTime<chrono::Utc>>,
    }
    impl ValenceEdgeReadsPayload {
        pub fn topic() -> &'static str {
            VALENCE_EDGE_READS_TOPIC
        }
        pub fn to_metric_emit(&self) -> ::spectra_core::MetricEmit {
            match self.ts {
                Some(ts) => ::spectra_core::MetricEmit::counter(
                    "valence_edge_reads",
                    self.labels.clone(),
                    self.delta,
                    ts,
                ),
                None => ::spectra_core::MetricEmit::counter(
                    "valence_edge_reads",
                    self.labels.clone(),
                    self.delta,
                    chrono::Utc::now(),
                ),
            }
        }
    }
    pub const VALENCE_EDGE_WRITES_TOPIC: &str = "spectra.metric.valence_edge_writes";
    #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
    pub struct ValenceEdgeWritesPayload {
        pub name: &'static str,
        pub labels: serde_json::Value,
        pub delta: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ts: Option<chrono::DateTime<chrono::Utc>>,
    }
    impl ValenceEdgeWritesPayload {
        pub fn topic() -> &'static str {
            VALENCE_EDGE_WRITES_TOPIC
        }
        pub fn to_metric_emit(&self) -> ::spectra_core::MetricEmit {
            match self.ts {
                Some(ts) => ::spectra_core::MetricEmit::counter(
                    "valence_edge_writes",
                    self.labels.clone(),
                    self.delta,
                    ts,
                ),
                None => ::spectra_core::MetricEmit::counter(
                    "valence_edge_writes",
                    self.labels.clone(),
                    self.delta,
                    chrono::Utc::now(),
                ),
            }
        }
    }
    pub const VALENCE_FIELD_REDACTIONS_TOPIC: &str = "spectra.metric.valence_field_redactions";
    #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
    pub struct ValenceFieldRedactionsPayload {
        pub name: &'static str,
        pub labels: serde_json::Value,
        pub delta: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ts: Option<chrono::DateTime<chrono::Utc>>,
    }
    impl ValenceFieldRedactionsPayload {
        pub fn topic() -> &'static str {
            VALENCE_FIELD_REDACTIONS_TOPIC
        }
        pub fn to_metric_emit(&self) -> ::spectra_core::MetricEmit {
            match self.ts {
                Some(ts) => ::spectra_core::MetricEmit::counter(
                    "valence_field_redactions",
                    self.labels.clone(),
                    self.delta,
                    ts,
                ),
                None => ::spectra_core::MetricEmit::counter(
                    "valence_field_redactions",
                    self.labels.clone(),
                    self.delta,
                    chrono::Utc::now(),
                ),
            }
        }
    }
    pub const VALENCE_MUTATION_WALL_MS_TOPIC: &str = "spectra.metric.valence_mutation_wall_ms";
    #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
    pub struct ValenceMutationWallMsPayload {
        pub name: &'static str,
        pub labels: serde_json::Value,
        pub delta: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ts: Option<chrono::DateTime<chrono::Utc>>,
    }
    impl ValenceMutationWallMsPayload {
        pub fn topic() -> &'static str {
            VALENCE_MUTATION_WALL_MS_TOPIC
        }
        pub fn to_metric_emit(&self) -> ::spectra_core::MetricEmit {
            match self.ts {
                Some(ts) => ::spectra_core::MetricEmit::counter(
                    "valence_mutation_wall_ms",
                    self.labels.clone(),
                    self.delta,
                    ts,
                ),
                None => ::spectra_core::MetricEmit::counter(
                    "valence_mutation_wall_ms",
                    self.labels.clone(),
                    self.delta,
                    chrono::Utc::now(),
                ),
            }
        }
    }
    pub const VALENCE_OWNERSHIP_TRANSFERS_TOPIC: &str =
        "spectra.metric.valence_ownership_transfers";
    #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
    pub struct ValenceOwnershipTransfersPayload {
        pub name: &'static str,
        pub labels: serde_json::Value,
        pub delta: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ts: Option<chrono::DateTime<chrono::Utc>>,
    }
    impl ValenceOwnershipTransfersPayload {
        pub fn topic() -> &'static str {
            VALENCE_OWNERSHIP_TRANSFERS_TOPIC
        }
        pub fn to_metric_emit(&self) -> ::spectra_core::MetricEmit {
            match self.ts {
                Some(ts) => ::spectra_core::MetricEmit::counter(
                    "valence_ownership_transfers",
                    self.labels.clone(),
                    self.delta,
                    ts,
                ),
                None => ::spectra_core::MetricEmit::counter(
                    "valence_ownership_transfers",
                    self.labels.clone(),
                    self.delta,
                    chrono::Utc::now(),
                ),
            }
        }
    }
    pub const VALENCE_PRIVACY_DENIALS_TOPIC: &str = "spectra.metric.valence_privacy_denials";
    #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
    pub struct ValencePrivacyDenialsPayload {
        pub name: &'static str,
        pub labels: serde_json::Value,
        pub delta: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ts: Option<chrono::DateTime<chrono::Utc>>,
    }
    impl ValencePrivacyDenialsPayload {
        pub fn topic() -> &'static str {
            VALENCE_PRIVACY_DENIALS_TOPIC
        }
        pub fn to_metric_emit(&self) -> ::spectra_core::MetricEmit {
            match self.ts {
                Some(ts) => ::spectra_core::MetricEmit::counter(
                    "valence_privacy_denials",
                    self.labels.clone(),
                    self.delta,
                    ts,
                ),
                None => ::spectra_core::MetricEmit::counter(
                    "valence_privacy_denials",
                    self.labels.clone(),
                    self.delta,
                    chrono::Utc::now(),
                ),
            }
        }
    }
    pub const VALENCE_PRIVACY_EVALUATIONS_TOPIC: &str =
        "spectra.metric.valence_privacy_evaluations";
    #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
    pub struct ValencePrivacyEvaluationsPayload {
        pub name: &'static str,
        pub labels: serde_json::Value,
        pub delta: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ts: Option<chrono::DateTime<chrono::Utc>>,
    }
    impl ValencePrivacyEvaluationsPayload {
        pub fn topic() -> &'static str {
            VALENCE_PRIVACY_EVALUATIONS_TOPIC
        }
        pub fn to_metric_emit(&self) -> ::spectra_core::MetricEmit {
            match self.ts {
                Some(ts) => ::spectra_core::MetricEmit::counter(
                    "valence_privacy_evaluations",
                    self.labels.clone(),
                    self.delta,
                    ts,
                ),
                None => ::spectra_core::MetricEmit::counter(
                    "valence_privacy_evaluations",
                    self.labels.clone(),
                    self.delta,
                    chrono::Utc::now(),
                ),
            }
        }
    }
    pub const VALENCE_QUERY_COMPILE_ERRORS_TOPIC: &str =
        "spectra.metric.valence_query_compile_errors";
    #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
    pub struct ValenceQueryCompileErrorsPayload {
        pub name: &'static str,
        pub labels: serde_json::Value,
        pub delta: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ts: Option<chrono::DateTime<chrono::Utc>>,
    }
    impl ValenceQueryCompileErrorsPayload {
        pub fn topic() -> &'static str {
            VALENCE_QUERY_COMPILE_ERRORS_TOPIC
        }
        pub fn to_metric_emit(&self) -> ::spectra_core::MetricEmit {
            match self.ts {
                Some(ts) => ::spectra_core::MetricEmit::counter(
                    "valence_query_compile_errors",
                    self.labels.clone(),
                    self.delta,
                    ts,
                ),
                None => ::spectra_core::MetricEmit::counter(
                    "valence_query_compile_errors",
                    self.labels.clone(),
                    self.delta,
                    chrono::Utc::now(),
                ),
            }
        }
    }
    pub const VALENCE_QUERY_EXECUTIONS_TOPIC: &str = "spectra.metric.valence_query_executions";
    #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
    pub struct ValenceQueryExecutionsPayload {
        pub name: &'static str,
        pub labels: serde_json::Value,
        pub delta: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ts: Option<chrono::DateTime<chrono::Utc>>,
    }
    impl ValenceQueryExecutionsPayload {
        pub fn topic() -> &'static str {
            VALENCE_QUERY_EXECUTIONS_TOPIC
        }
        pub fn to_metric_emit(&self) -> ::spectra_core::MetricEmit {
            match self.ts {
                Some(ts) => ::spectra_core::MetricEmit::counter(
                    "valence_query_executions",
                    self.labels.clone(),
                    self.delta,
                    ts,
                ),
                None => ::spectra_core::MetricEmit::counter(
                    "valence_query_executions",
                    self.labels.clone(),
                    self.delta,
                    chrono::Utc::now(),
                ),
            }
        }
    }
    pub const VALENCE_QUERY_RESULTS_CAPPED_TOPIC: &str =
        "spectra.metric.valence_query_results_capped";
    #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
    pub struct ValenceQueryResultsCappedPayload {
        pub name: &'static str,
        pub labels: serde_json::Value,
        pub delta: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ts: Option<chrono::DateTime<chrono::Utc>>,
    }
    impl ValenceQueryResultsCappedPayload {
        pub fn topic() -> &'static str {
            VALENCE_QUERY_RESULTS_CAPPED_TOPIC
        }
        pub fn to_metric_emit(&self) -> ::spectra_core::MetricEmit {
            match self.ts {
                Some(ts) => ::spectra_core::MetricEmit::counter(
                    "valence_query_results_capped",
                    self.labels.clone(),
                    self.delta,
                    ts,
                ),
                None => ::spectra_core::MetricEmit::counter(
                    "valence_query_results_capped",
                    self.labels.clone(),
                    self.delta,
                    chrono::Utc::now(),
                ),
            }
        }
    }
    pub const VALENCE_QUERY_ROWS_FILTERED_TOPIC: &str =
        "spectra.metric.valence_query_rows_filtered";
    #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
    pub struct ValenceQueryRowsFilteredPayload {
        pub name: &'static str,
        pub labels: serde_json::Value,
        pub delta: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ts: Option<chrono::DateTime<chrono::Utc>>,
    }
    impl ValenceQueryRowsFilteredPayload {
        pub fn topic() -> &'static str {
            VALENCE_QUERY_ROWS_FILTERED_TOPIC
        }
        pub fn to_metric_emit(&self) -> ::spectra_core::MetricEmit {
            match self.ts {
                Some(ts) => ::spectra_core::MetricEmit::counter(
                    "valence_query_rows_filtered",
                    self.labels.clone(),
                    self.delta,
                    ts,
                ),
                None => ::spectra_core::MetricEmit::counter(
                    "valence_query_rows_filtered",
                    self.labels.clone(),
                    self.delta,
                    chrono::Utc::now(),
                ),
            }
        }
    }
    pub const VALENCE_QUERY_WALL_MS_TOPIC: &str = "spectra.metric.valence_query_wall_ms";
    #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
    pub struct ValenceQueryWallMsPayload {
        pub name: &'static str,
        pub labels: serde_json::Value,
        pub delta: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ts: Option<chrono::DateTime<chrono::Utc>>,
    }
    impl ValenceQueryWallMsPayload {
        pub fn topic() -> &'static str {
            VALENCE_QUERY_WALL_MS_TOPIC
        }
        pub fn to_metric_emit(&self) -> ::spectra_core::MetricEmit {
            match self.ts {
                Some(ts) => ::spectra_core::MetricEmit::counter(
                    "valence_query_wall_ms",
                    self.labels.clone(),
                    self.delta,
                    ts,
                ),
                None => ::spectra_core::MetricEmit::counter(
                    "valence_query_wall_ms",
                    self.labels.clone(),
                    self.delta,
                    chrono::Utc::now(),
                ),
            }
        }
    }
    pub const VALENCE_ROUTER_RESOLVE_ERRORS_TOPIC: &str =
        "spectra.metric.valence_router_resolve_errors";
    #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
    pub struct ValenceRouterResolveErrorsPayload {
        pub name: &'static str,
        pub labels: serde_json::Value,
        pub delta: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ts: Option<chrono::DateTime<chrono::Utc>>,
    }
    impl ValenceRouterResolveErrorsPayload {
        pub fn topic() -> &'static str {
            VALENCE_ROUTER_RESOLVE_ERRORS_TOPIC
        }
        pub fn to_metric_emit(&self) -> ::spectra_core::MetricEmit {
            match self.ts {
                Some(ts) => ::spectra_core::MetricEmit::counter(
                    "valence_router_resolve_errors",
                    self.labels.clone(),
                    self.delta,
                    ts,
                ),
                None => ::spectra_core::MetricEmit::counter(
                    "valence_router_resolve_errors",
                    self.labels.clone(),
                    self.delta,
                    chrono::Utc::now(),
                ),
            }
        }
    }
    pub const VALENCE_SIDE_EFFECTS_TOPIC: &str = "spectra.metric.valence_side_effects";
    #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
    pub struct ValenceSideEffectsPayload {
        pub name: &'static str,
        pub labels: serde_json::Value,
        pub delta: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ts: Option<chrono::DateTime<chrono::Utc>>,
    }
    impl ValenceSideEffectsPayload {
        pub fn topic() -> &'static str {
            VALENCE_SIDE_EFFECTS_TOPIC
        }
        pub fn to_metric_emit(&self) -> ::spectra_core::MetricEmit {
            match self.ts {
                Some(ts) => ::spectra_core::MetricEmit::counter(
                    "valence_side_effects",
                    self.labels.clone(),
                    self.delta,
                    ts,
                ),
                None => ::spectra_core::MetricEmit::counter(
                    "valence_side_effects",
                    self.labels.clone(),
                    self.delta,
                    chrono::Utc::now(),
                ),
            }
        }
    }
    pub const VALENCE_UNIQUE_VIOLATIONS_TOPIC: &str = "spectra.metric.valence_unique_violations";
    #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
    pub struct ValenceUniqueViolationsPayload {
        pub name: &'static str,
        pub labels: serde_json::Value,
        pub delta: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ts: Option<chrono::DateTime<chrono::Utc>>,
    }
    impl ValenceUniqueViolationsPayload {
        pub fn topic() -> &'static str {
            VALENCE_UNIQUE_VIOLATIONS_TOPIC
        }
        pub fn to_metric_emit(&self) -> ::spectra_core::MetricEmit {
            match self.ts {
                Some(ts) => ::spectra_core::MetricEmit::counter(
                    "valence_unique_violations",
                    self.labels.clone(),
                    self.delta,
                    ts,
                ),
                None => ::spectra_core::MetricEmit::counter(
                    "valence_unique_violations",
                    self.labels.clone(),
                    self.delta,
                    chrono::Utc::now(),
                ),
            }
        }
    }
    pub const VALENCE_VALIDATION_FAILURES_TOPIC: &str =
        "spectra.metric.valence_validation_failures";
    #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
    pub struct ValenceValidationFailuresPayload {
        pub name: &'static str,
        pub labels: serde_json::Value,
        pub delta: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ts: Option<chrono::DateTime<chrono::Utc>>,
    }
    impl ValenceValidationFailuresPayload {
        pub fn topic() -> &'static str {
            VALENCE_VALIDATION_FAILURES_TOPIC
        }
        pub fn to_metric_emit(&self) -> ::spectra_core::MetricEmit {
            match self.ts {
                Some(ts) => ::spectra_core::MetricEmit::counter(
                    "valence_validation_failures",
                    self.labels.clone(),
                    self.delta,
                    ts,
                ),
                None => ::spectra_core::MetricEmit::counter(
                    "valence_validation_failures",
                    self.labels.clone(),
                    self.delta,
                    chrono::Utc::now(),
                ),
            }
        }
    }
}
pub mod sink_forward {
    fn field_str(fields: &serde_json::Value, key: &str) -> String {
        fields
            .get(key)
            .and_then(|v| match v {
                serde_json::Value::String(s) => Some(s.clone()),
                serde_json::Value::Bool(b) => Some(b.to_string()),
                serde_json::Value::Number(n) => Some(n.to_string()),
                _ => None,
            })
            .unwrap_or_default()
    }
    #[allow(unused_variables)]
    pub fn forward_counter(
        name: String,
        labels: serde_json::Value,
        delta: i64,
        ts: chrono::DateTime<chrono::Utc>,
    ) {
        match name.as_str() {
            "valence_db_errors" => {
                use super::helpers::ValenceDbErrorsRecorder;
                ValenceDbErrorsRecorder::record_at(delta, labels, ts);
            }
            "valence_db_reads" => {
                use super::helpers::ValenceDbReadsRecorder;
                ValenceDbReadsRecorder::record_at(delta, labels, ts);
            }
            "valence_db_wall_ms" => {
                use super::helpers::ValenceDbWallMsRecorder;
                ValenceDbWallMsRecorder::record_at(delta, labels, ts);
            }
            "valence_db_writes" => {
                use super::helpers::ValenceDbWritesRecorder;
                ValenceDbWritesRecorder::record_at(delta, labels, ts);
            }
            "valence_deletion_dag_max_depth" => {
                use super::helpers::ValenceDeletionDagMaxDepthRecorder;
                ValenceDeletionDagMaxDepthRecorder::record_at(delta, labels, ts);
            }
            "valence_deletion_dag_nodes" => {
                use super::helpers::ValenceDeletionDagNodesRecorder;
                ValenceDeletionDagNodesRecorder::record_at(delta, labels, ts);
            }
            "valence_deletion_restrict_blocked" => {
                use super::helpers::ValenceDeletionRestrictBlockedRecorder;
                ValenceDeletionRestrictBlockedRecorder::record_at(delta, labels, ts);
            }
            "valence_deletion_run_queued" => {
                use super::helpers::ValenceDeletionRunQueuedRecorder;
                ValenceDeletionRunQueuedRecorder::record_at(delta, labels, ts);
            }
            "valence_deserialize_errors" => {
                use super::helpers::ValenceDeserializeErrorsRecorder;
                ValenceDeserializeErrorsRecorder::record_at(delta, labels, ts);
            }
            "valence_edge_reads" => {
                use super::helpers::ValenceEdgeReadsRecorder;
                ValenceEdgeReadsRecorder::record_at(delta, labels, ts);
            }
            "valence_edge_writes" => {
                use super::helpers::ValenceEdgeWritesRecorder;
                ValenceEdgeWritesRecorder::record_at(delta, labels, ts);
            }
            "valence_field_redactions" => {
                use super::helpers::ValenceFieldRedactionsRecorder;
                ValenceFieldRedactionsRecorder::record_at(delta, labels, ts);
            }
            "valence_mutation_wall_ms" => {
                use super::helpers::ValenceMutationWallMsRecorder;
                ValenceMutationWallMsRecorder::record_at(delta, labels, ts);
            }
            "valence_ownership_transfers" => {
                use super::helpers::ValenceOwnershipTransfersRecorder;
                ValenceOwnershipTransfersRecorder::record_at(delta, labels, ts);
            }
            "valence_privacy_denials" => {
                use super::helpers::ValencePrivacyDenialsRecorder;
                ValencePrivacyDenialsRecorder::record_at(delta, labels, ts);
            }
            "valence_privacy_evaluations" => {
                use super::helpers::ValencePrivacyEvaluationsRecorder;
                ValencePrivacyEvaluationsRecorder::record_at(delta, labels, ts);
            }
            "valence_query_compile_errors" => {
                use super::helpers::ValenceQueryCompileErrorsRecorder;
                ValenceQueryCompileErrorsRecorder::record_at(delta, labels, ts);
            }
            "valence_query_executions" => {
                use super::helpers::ValenceQueryExecutionsRecorder;
                ValenceQueryExecutionsRecorder::record_at(delta, labels, ts);
            }
            "valence_query_results_capped" => {
                use super::helpers::ValenceQueryResultsCappedRecorder;
                ValenceQueryResultsCappedRecorder::record_at(delta, labels, ts);
            }
            "valence_query_rows_filtered" => {
                use super::helpers::ValenceQueryRowsFilteredRecorder;
                ValenceQueryRowsFilteredRecorder::record_at(delta, labels, ts);
            }
            "valence_query_wall_ms" => {
                use super::helpers::ValenceQueryWallMsRecorder;
                ValenceQueryWallMsRecorder::record_at(delta, labels, ts);
            }
            "valence_router_resolve_errors" => {
                use super::helpers::ValenceRouterResolveErrorsRecorder;
                ValenceRouterResolveErrorsRecorder::record_at(delta, labels, ts);
            }
            "valence_side_effects" => {
                use super::helpers::ValenceSideEffectsRecorder;
                ValenceSideEffectsRecorder::record_at(delta, labels, ts);
            }
            "valence_unique_violations" => {
                use super::helpers::ValenceUniqueViolationsRecorder;
                ValenceUniqueViolationsRecorder::record_at(delta, labels, ts);
            }
            "valence_validation_failures" => {
                use super::helpers::ValenceValidationFailuresRecorder;
                ValenceValidationFailuresRecorder::record_at(delta, labels, ts);
            }
            _ => {}
        }
    }
    #[allow(unused_variables)]
    pub fn forward_event(
        table: String,
        fields: serde_json::Value,
        ts: chrono::DateTime<chrono::Utc>,
    ) {
        match table.as_str() {
            "valence_deletion_log" => {
                use super::helpers::ValenceDeletionLogLogger;
                ValenceDeletionLogLogger::log_at(
                    field_str(&fields, "outcome"),
                    field_str(&fields, "root_table"),
                    field_str(&fields, "root_record_id"),
                    field_str(&fields, "node_count"),
                    field_str(&fields, "max_depth"),
                    field_str(&fields, "restrict_count"),
                    field_str(&fields, "trait_expansion_count"),
                    ts,
                );
            }
            "valence_error_log" => {
                use super::helpers::ValenceErrorLogLogger;
                ValenceErrorLogLogger::log_at(
                    field_str(&fields, "source"),
                    field_str(&fields, "operation"),
                    field_str(&fields, "table"),
                    field_str(&fields, "database_type"),
                    field_str(&fields, "message"),
                    ts,
                );
            }
            "valence_ownership_log" => {
                use super::helpers::ValenceOwnershipLogLogger;
                ValenceOwnershipLogLogger::log_at(
                    field_str(&fields, "table"),
                    field_str(&fields, "record_id"),
                    field_str(&fields, "from_owner_id"),
                    field_str(&fields, "from_owner_type"),
                    field_str(&fields, "to_owner_id"),
                    field_str(&fields, "to_owner_type"),
                    field_str(&fields, "actor"),
                    ts,
                );
            }
            "valence_privacy_eval" => {
                use super::helpers::ValencePrivacyEvalLogger;
                ValencePrivacyEvalLogger::log_at(
                    field_str(&fields, "policy"),
                    field_str(&fields, "actor_kind"),
                    field_str(&fields, "actor_id"),
                    field_str(&fields, "operation"),
                    field_str(&fields, "table"),
                    field_str(&fields, "rule_phase"),
                    field_str(&fields, "matched"),
                    field_str(&fields, "check_outcome"),
                    ts,
                );
            }
            "valence_query_log" => {
                use super::helpers::ValenceQueryLogLogger;
                ValenceQueryLogLogger::log_at(
                    field_str(&fields, "outcome"),
                    field_str(&fields, "query_target"),
                    field_str(&fields, "trait_name"),
                    field_str(&fields, "primary_table"),
                    field_str(&fields, "implementor_count"),
                    field_str(&fields, "hop_count"),
                    field_str(&fields, "has_limit"),
                    field_str(&fields, "limit"),
                    field_str(&fields, "offset"),
                    field_str(&fields, "rows_db"),
                    field_str(&fields, "rows_after_privacy"),
                    field_str(&fields, "rows_after_pending_deletion"),
                    field_str(&fields, "wall_ms"),
                    field_str(&fields, "compile_error"),
                    field_str(&fields, "caller"),
                    ts,
                );
            }
            "valence_slow_op" => {
                use super::helpers::ValenceSlowOpLogger;
                ValenceSlowOpLogger::log_at(
                    field_str(&fields, "operation"),
                    field_str(&fields, "table"),
                    field_str(&fields, "op"),
                    field_str(&fields, "database_type"),
                    field_str(&fields, "wall_ms"),
                    field_str(&fields, "record_id"),
                    ts,
                );
            }
            _ => {}
        }
    }
}
