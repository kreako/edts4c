ALTER TABLE domains DROP CONSTRAINT domains_rank_key;
ALTER TABLE components DROP CONSTRAINT components_domain_id_rank_key;
ALTER TABLE competencies DROP CONSTRAINT competencies_component_id_rank_key;
