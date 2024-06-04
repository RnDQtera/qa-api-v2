<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Tag Replace</name>
   <tag></tag>
   <elementGuidId>af68b389-2f5f-493a-8521-bccf727bf6eb</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;data\&quot;: [\n    {\n      \&quot;asset\&quot;: \&quot;\u003cstring\u003e\&quot;,\n      \&quot;qrCode\&quot;: \&quot;\u003cstring\u003e\&quot;,\n      \&quot;rfidCode\&quot;: \&quot;\u003cstring\u003e\&quot;,\n      \&quot;rfidIsVerified\&quot;: \&quot;\u003cboolean\u003e\&quot;,\n      \&quot;qrIsVerified\&quot;: \&quot;\u003cboolean\u003e\&quot;\n    },\n    {\n      \&quot;asset\&quot;: \&quot;\u003cstring\u003e\&quot;,\n      \&quot;qrCode\&quot;: \&quot;\u003cstring\u003e\&quot;,\n      \&quot;rfidCode\&quot;: \&quot;\u003cstring\u003e\&quot;,\n      \&quot;rfidIsVerified\&quot;: \&quot;\u003cboolean\u003e\&quot;,\n      \&quot;qrIsVerified\&quot;: \&quot;\u003cboolean\u003e\&quot;\n    }\n  ]\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>95fb14d2-2c9d-4429-b34a-42dc83f60db3</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.4.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${baseUrl}/v2/tag-transaction/replace</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.baseUrl</defaultValue>
      <description></description>
      <id>567f27e9-1f92-4094-96e6-52592b2198c5</id>
      <masked>false</masked>
      <name>baseUrl</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
