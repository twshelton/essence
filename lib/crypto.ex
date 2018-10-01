  defmodule Crypto do
        def create_key(wallet_handle,key_json), do: Indy.create_key(wallet_handle,key_json)
  def set_key_metadata(wallet_handle,verkey,metadata), do: Indy.set_key_metadata(wallet_handle,verkey,metadata)
  def get_key_metadata(wallet_handle,verkey), do: Indy.get_key_metadata(wallet_handle,verkey)
  def crypto_sign(wallet_handle,signer_vk,message_raw,message_len), do: Indy.crypto_sign(wallet_handle,signer_vk,message_raw,message_len)
  def crypto_verify(signer_vk,message_raw,message_len,signature_raw,signature_len), do: Indy.crypto_verify(signer_vk,message_raw,message_len,signature_raw,signature_len)
  def crypto_auth_crypt(wallet_handle,sender_vk,recipient_vk,msg_data,msg_len), do: Indy.crypto_auth_crypt(wallet_handle,sender_vk,recipient_vk,msg_data,msg_len)
  def crypto_auth_decrypt(wallet_handle,recipient_vk,encrypted_msg,encrypted_len), do: Indy.crypto_auth_decrypt(wallet_handle,recipient_vk,encrypted_msg,encrypted_len)
  def crypto_anon_crypt(recipient_vk,msg_data,msg_len), do: Indy.crypto_anon_crypt(recipient_vk,msg_data,msg_len)
  def crypto_anon_decrypt(wallet_handle,recipient_vk,encrypted_msg,encrypted_len), do: Indy.crypto_anon_decrypt(wallet_handle,recipient_vk,encrypted_msg,encrypted_len)

  end
